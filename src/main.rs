use clap::Parser;
use localhost::{config::Args, create_and_bind_socket, epoll::Epoll, Config};

/**
 * Main entry point for the localhost HTTP server.
 *
 * Initializes the server socket, creates an epoll instance,
 * and runs the main event loop to accept incoming connections.
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

    loop {
        match epoll.wait(-1) {
            Ok(events) => {
                for event in events {
                    let fd = event.fd;
                    if fd == server_socket.fd() {
                        println!("Server socket is readable (incoming connection)");
                    } else {
                        println!("Client socket {} is readable", fd);
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
