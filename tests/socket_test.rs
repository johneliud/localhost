use libc::{sockaddr, sockaddr_in, socklen_t};
use localhost::{create_and_bind_socket, socket::Socket, SocketError};
use std::net::Ipv4Addr;

#[test]
fn test_socket_creation() {
    let sock = Socket::new();
    assert!(sock.is_ok());
    let sock = sock.unwrap();
    assert!(sock.fd() >= 0);
}

#[test]
fn test_socket_reuseaddr() {
    let sock = Socket::new().unwrap();
    let result = sock.set_reuseaddr();
    assert!(result.is_ok());
}

#[test]
fn test_bind_and_listen() {
    let sock = Socket::new().unwrap();
    sock.set_reuseaddr().unwrap();

    let ip = Ipv4Addr::new(0, 0, 0, 0);
    let addr = sockaddr_in {
        sin_family: libc::AF_INET as u16,
        sin_port: 0u16.to_be(),
        sin_addr: libc::in_addr {
            s_addr: u32::from(ip),
        },
        sin_zero: [0; 8],
    };

    let bind_result = unsafe {
        libc::bind(
            sock.fd(),
            &addr as *const sockaddr_in as *const sockaddr,
            std::mem::size_of::<sockaddr_in>() as socklen_t,
        )
    };
    assert_eq!(bind_result, 0, "Bind should succeed");

    let listen_result = sock.listen(128);
    assert!(listen_result.is_ok());
}

#[test]
fn test_nonblocking_socket() {
    let sock = Socket::new().unwrap();
    let result = sock.set_nonblocking();
    assert!(result.is_ok());
}

#[test]
fn test_create_and_bind_socket() {
    let result = create_and_bind_socket("127.0.0.1", 18081);
    assert!(result.is_ok());
}

#[test]
fn test_socket_error_display() {
    let err = SocketError::Create;
    assert!(!err.to_string().is_empty());
}
