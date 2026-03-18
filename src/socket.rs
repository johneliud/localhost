use libc::{c_int, sockaddr, sockaddr_in, socklen_t};
use std::net::Ipv4Addr;

/**
 * Represents a TCP socket with its file descriptor.
 */
pub struct Socket {
    fd: c_int,
}

impl Socket {
    /**
     * Creates a new TCP socket.
     *
     * Returns Ok(Socket) on success or Err(SocketError) on failure.
     */
    pub fn new() -> Result<Self, SocketError> {
        unsafe {
            let sock = libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0);
            if sock < 0 {
                return Err(SocketError::Create);
            }
            Ok(Self { fd: sock })
        }
    }

    /**
     * Sets the SO_REUSEADDR option on the socket.
     *
     * Allows the socket to be bound to an address that is already in use.
     * Returns Ok(()) on success or Err(SocketError) on failure.
     */
    pub fn set_reuseaddr(&self) -> Result<(), SocketError> {
        unsafe {
            let opt: c_int = 1;
            let result = libc::setsockopt(
                self.fd,
                libc::SOL_SOCKET,
                libc::SO_REUSEADDR,
                &opt as *const c_int as *const libc::c_void,
                std::mem::size_of::<c_int>() as socklen_t,
            );
            if result < 0 {
                return Err(SocketError::SetReuseAddr);
            }
            Ok(())
        }
    }

    /**
     * Binds the socket to the specified host and port.
     *
     * # Arguments
     * * `host` - The IP address to bind to
     * * `port` - The port number to bind to
     *
     * Returns Ok(()) on success or Err(SocketError) on failure.
     */
    pub fn bind(&self, host: &str, port: u16) -> Result<(), SocketError> {
        let ip: Ipv4Addr = host.parse().map_err(|_| SocketError::InvalidAddress)?;

        let addr = sockaddr_in {
            sin_family: libc::AF_INET as u16,
            sin_port: port.to_be(),
            sin_addr: libc::in_addr {
                s_addr: u32::from(ip).to_be(),
            },
            sin_zero: [0; 8],
        };

        unsafe {
            let result = libc::bind(
                self.fd,
                &addr as *const sockaddr_in as *const sockaddr,
                std::mem::size_of::<sockaddr_in>() as socklen_t,
            );
            if result < 0 {
                return Err(SocketError::Bind);
            }
        }
        Ok(())
    }

    /**
     * Marks the socket as a passive socket that will accept incoming connections.
     *
     * # Arguments
     * * `backlog` - The maximum length of the pending connection queue
     *
     * Returns Ok(()) on success or Err(SocketError) on failure.
     */
    pub fn listen(&self, backlog: u32) -> Result<(), SocketError> {
        unsafe {
            let result = libc::listen(self.fd, backlog as c_int);
            if result < 0 {
                return Err(SocketError::Listen);
            }
        }
        Ok(())
    }

    /**
     * Sets the socket to non-blocking mode.
     *
     * Operations on the socket will return immediately without blocking.
     * Returns Ok(()) on success or Err(SocketError) on failure.
     */
    pub fn set_nonblocking(&self) -> Result<(), SocketError> {
        unsafe {
            let flags = libc::fcntl(self.fd, libc::F_GETFL);
            if flags < 0 {
                return Err(SocketError::SetNonBlocking);
            }
            let result = libc::fcntl(self.fd, libc::F_SETFL, flags | libc::O_NONBLOCK);
            if result < 0 {
                return Err(SocketError::SetNonBlocking);
            }
        }
        Ok(())
    }

    /**
     * Returns the raw file descriptor for the socket.
     */
    pub fn fd(&self) -> c_int {
        self.fd
    }

    /**
     * Closes the socket file descriptor.
     */
    pub fn close(&self) {
        unsafe {
            libc::close(self.fd);
        }
    }
}

impl Drop for Socket {
    /**
     * Automatically closes the socket when the Socket struct goes out of scope.
     */
    fn drop(&mut self) {
        self.close();
    }
}

/**
 * Error types that can occur when working with sockets.
 */
#[derive(Debug, Clone, Copy)]
pub enum SocketError {
    Create,
    SetReuseAddr,
    InvalidAddress,
    Bind,
    Listen,
    SetNonBlocking,
}

impl std::fmt::Display for SocketError {
    /**
     * Formats the error for display purposes.
     */
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SocketError::Create => write!(f, "Failed to create socket"),
            SocketError::SetReuseAddr => write!(f, "Failed to set SO_REUSEADDR"),
            SocketError::InvalidAddress => write!(f, "Invalid IP address"),
            SocketError::Bind => write!(f, "Failed to bind socket"),
            SocketError::Listen => write!(f, "Failed to listen on socket"),
            SocketError::SetNonBlocking => write!(f, "Failed to set non-blocking"),
        }
    }
}

impl std::error::Error for SocketError {}

/**
 * Creates a fully configured server socket that is bound, listening, and non-blocking.
 *
 * # Arguments
 * * `host` - The IP address to bind to
 * * `port` - The port number to bind to
 *
 * Returns Ok(Socket) on success or Err(SocketError) on failure.
 */
pub fn create_and_bind_socket(host: &str, port: u16) -> Result<Socket, SocketError> {
    let sock = Socket::new()?;
    sock.set_reuseaddr()?;
    sock.bind(host, port)?;
    sock.listen(128)?;
    sock.set_nonblocking()?;
    Ok(sock)
}
