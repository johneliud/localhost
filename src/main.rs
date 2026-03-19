use clap::Parser;
use localhost::{
    config::Args,
    connection::{parse_epoll_event, ConnectionManager, Event},
    create_and_bind_socket,
    epoll::Epoll,
    Config, Socket,
};

/**
 * Main entry point for the localhost HTTP server.
 *
 * Initializes the server socket, creates an epoll instance,
 * and runs the main event loop to accept and handle connections.
 */
fn main() {
    let args = Args::parse();
    let config = Config::from_args(args);

    println!("Starting server on {}", config.address());

    let server_socket = match create_and_bind_socket(&config.host, config.port) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to create server socket: {}", e);
            std::process::exit(1);
        }
    };
    println!("Server socket created with fd: {}", server_socket.fd());

    let mut epoll = match Epoll::new() {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Failed to create epoll instance: {}", e);
            std::process::exit(1);
        }
    };
    println!("Epoll instance created with fd: {}", epoll.fd());

    if let Err(e) = epoll.add_read(server_socket.fd()) {
        eprintln!("Failed to add server socket to epoll: {}", e);
        std::process::exit(1);
    }

    let mut connection_manager = ConnectionManager::new();

    println!("Entering main event loop...");

    loop {
        match epoll.wait(-1) {
            Ok(events) => {
                for epoll_event in events {
                    let events = parse_epoll_event(epoll_event.fd, epoll_event.events);

                    for event in events {
                        match event {
                            Event::Accept(fd) if fd == server_socket.fd() => {
                                handle_accept(&server_socket, &mut epoll, &mut connection_manager);
                            }
                            Event::Accept(_) => {}
                            Event::Read(fd) => {
                                handle_read(&mut epoll, &mut connection_manager, fd);
                            }
                            Event::Write(fd) => {
                                handle_write(&mut epoll, &mut connection_manager, fd);
                            }
                            Event::HangUp(fd) => {
                                handle_hangup(&mut epoll, &mut connection_manager, fd);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("epoll_wait error: {}", e);
                break;
            }
        }
    }

    println!("Server shutting down");
}

/**
 * Handles an incoming connection on the server socket.
 *
 * # Arguments
 * * `server_socket` - The server socket to accept from
 * * `epoll` - The epoll instance
 * * `connection_manager` - The connection manager
 */
fn handle_accept(
    server_socket: &Socket,
    epoll: &mut Epoll,
    connection_manager: &mut ConnectionManager,
) {
    loop {
        match ConnectionManager::accept_connection(server_socket) {
            Ok(connection) => {
                println!(
                    "Accepted new connection with fd: {}",
                    connection.socket.fd()
                );
                if let Err(e) = connection_manager.add_connection(epoll, connection) {
                    eprintln!("Failed to add connection to epoll: {}", e);
                }
            }
            Err(localhost::ConnectionError::WouldBlock) => {
                break;
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
                break;
            }
        }
    }
}

/**
 * Handles a read event on a client socket.
 *
 * # Arguments
 * * `epoll` - The epoll instance
 * * `connection_manager` - The connection manager
 * * `fd` - The file descriptor to read from
 */
fn handle_read(epoll: &mut Epoll, connection_manager: &mut ConnectionManager, fd: libc::c_int) {
    if let Some(connection) = connection_manager.get_connection(fd) {
        let mut buffer = [0u8; 8192];
        let bytes_read =
            unsafe { libc::read(fd, buffer.as_mut_ptr() as *mut libc::c_void, buffer.len()) };

        if bytes_read > 0 {
            connection
                .read_buffer
                .extend_from_slice(&buffer[..bytes_read as usize]);
            println!("Read {} bytes from fd {}", bytes_read, fd);
        } else if bytes_read == 0 {
            println!("Client fd {} closed connection", fd);
            let _ = connection_manager.remove_connection(epoll, fd);
        } else {
            let errno = std::io::Error::last_os_error().raw_os_error();
            if errno != Some(libc::EAGAIN) && errno != Some(libc::EWOULDBLOCK) {
                eprintln!("Read error on fd {}: {:?}", fd, errno);
                let _ = connection_manager.remove_connection(epoll, fd);
            }
        }
    }
}

/**
 * Handles a write event on a client socket.
 *
 * # Arguments
 * * `epoll` - The epoll instance
 * * `connection_manager` - The connection manager
 * * `fd` - The file descriptor to write to
 */
fn handle_write(epoll: &mut Epoll, connection_manager: &mut ConnectionManager, fd: libc::c_int) {
    if let Some(connection) = connection_manager.get_connection(fd)
        && !connection.write_buffer.is_empty()
    {
        let chunk = &connection.write_buffer[connection.bytes_written..];

        let bytes_written =
            unsafe { libc::write(fd, chunk.as_ptr() as *const libc::c_void, chunk.len()) };

        if bytes_written > 0 {
            connection.bytes_written += bytes_written as usize;
            println!(
                "Wrote {} bytes to fd {} (total: {}/{})",
                bytes_written,
                fd,
                connection.bytes_written,
                connection.write_buffer.len()
            );
        } else if bytes_written < 0 {
            let errno = std::io::Error::last_os_error().raw_os_error();
            if errno != Some(libc::EAGAIN) && errno != Some(libc::EWOULDBLOCK) {
                eprintln!("Write error on fd {}: {:?}", fd, errno);
                let _ = connection_manager.remove_connection(epoll, fd);
            }
        }
    }
}
