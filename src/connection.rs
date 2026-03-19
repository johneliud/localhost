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

impl ConnectionManager {
    /**
     * Creates a new ConnectionManager.
     */
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
        }
    }

    /**
     * Accepts a new connection on the server socket.
     *
     * # Arguments
     * * `server_socket` - The server socket to accept from
     *
     * Returns Ok(Connection) on success or Err(ConnectionError) on failure.
     */
    pub fn accept_connection(server_socket: &Socket) -> Result<Connection, ConnectionError> {
        let mut addr_size = std::mem::size_of::<sockaddr_in>() as socklen_t;
        let mut addr: sockaddr_in = unsafe { std::mem::zeroed() };

        let client_fd = unsafe {
            libc::accept(
                server_socket.fd(),
                &mut addr as *mut sockaddr_in as *mut sockaddr,
                &mut addr_size,
            )
        };

        if client_fd < 0 {
            let errno = std::io::Error::last_os_error().raw_os_error();
            if errno == Some(libc::EAGAIN) || errno == Some(libc::EWOULDBLOCK) {
                return Err(ConnectionError::WouldBlock);
            }
            return Err(ConnectionError::Accept);
        }

        let socket = Socket::from_fd(client_fd)?;
        socket.set_nonblocking()?;

        Ok(Connection {
            socket,
            state: ConnectionState::Reading,
            read_buffer: Vec::new(),
            write_buffer: Vec::new(),
            bytes_written: 0,
        })
    }

    /**
     * Adds a connection to the manager and registers it with epoll.
     *
     * # Arguments
     * * `epoll` - The epoll instance to register with
     * * `connection` - The connection to add
     *
     * Returns Ok(()) on success or Err(ConnectionError) on failure.
     */
    pub fn add_connection(
        &mut self,
        epoll: &mut Epoll,
        connection: Connection,
    ) -> Result<(), ConnectionError> {
        let fd = connection.socket.fd();
        epoll
            .add(fd, EPOLLIN as u32)
            .map_err(ConnectionError::Epoll)?;
        self.connections.insert(fd, connection);
        Ok(())
    }

    /**
     * Removes a connection from the manager and closes it.
     *
     * # Arguments
     * * `epoll` - The epoll instance to unregister from
     * * `fd` - The file descriptor to remove
     *
     * Returns Ok(()) on success or Err(ConnectionError) on failure.
     */
    pub fn remove_connection(
        &mut self,
        epoll: &mut Epoll,
        fd: c_int,
    ) -> Result<(), ConnectionError> {
        epoll.remove(fd).map_err(ConnectionError::Epoll)?;
        self.connections.remove(&fd);
        Ok(())
    }

    /**
     * Gets a mutable reference to a connection by file descriptor.
     *
     * # Arguments
     * * `fd` - The file descriptor to look up
     *
     * Returns Some(&mut Connection) or None if not found.
     */
    pub fn get_connection(&mut self, fd: c_int) -> Option<&mut Connection> {
        self.connections.get_mut(&fd)
    }

    /**
     * Returns the number of active connections.
     */
    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }
}
