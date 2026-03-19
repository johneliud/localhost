use libc::{c_int, sockaddr, sockaddr_in, socklen_t, EPOLLIN, EPOLLOUT};
use std::collections::HashMap;

use crate::epoll::{Epoll, EpollError};
use crate::socket::{Socket, SocketError};

/**
 * Represents the state of a client connection.
 * 
 * Reading: Connection is open and ready to receive data.
 * Processing: Connection is processing the request.
 * Writing: Connection is sending a response.
 * Closed: Connection is closed.
 */
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnectionState {
    Reading,
    Processing,
    Writing,
    Closed,
}

/**
 * Represents a client connection with its socket and state.
 * 
 * socket: The socket for this connection.
 * state: Current state of the connection.
 * read_buffer: Buffer for incoming data.
 * write_buffer: Buffer for outgoing data.
 * bytes_written: Number of bytes written so far.
 */
#[derive(Debug)]
pub struct Connection {
    pub socket: Socket,
    pub state: ConnectionState,
    pub read_buffer: Vec<u8>,
    pub write_buffer: Vec<u8>,
    pub bytes_written: usize,
}

/**
 * Manages all active client connections.
 */
pub struct ConnectionManager {
    /** Map of file descriptor to Connection. */
    connections: HashMap<c_int, Connection>,
}
