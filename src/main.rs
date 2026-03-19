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
