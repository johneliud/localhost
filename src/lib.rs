/**
 * Localhost HTTP Server Library
 *
 * A lightweight, non-blocking HTTP server written in Rust using epoll.
 */
pub mod config;
pub mod epoll;
pub mod socket;

pub use config::{Args, Config};
pub use epoll::{Epoll, EpollError, EpollEvent, MAX_EVENTS};
pub use socket::{create_and_bind_socket, Socket, SocketError};
