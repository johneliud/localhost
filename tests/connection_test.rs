use libc::EPOLLIN;
use localhost::{
    connection::{parse_epoll_event, ConnectionManager, ConnectionState, Event},
    socket::Socket,
};

#[test]
fn test_connection_manager_new() {
    let manager = ConnectionManager::new();
    assert_eq!(manager.connection_count(), 0);
}

#[test]
fn test_connection_state() {
    let state = ConnectionState::Reading;
    assert_eq!(state, ConnectionState::Reading);

    let state = ConnectionState::Closed;
    assert_eq!(state, ConnectionState::Closed);
}

#[test]
fn test_socket_from_fd() {
    let sock = Socket::new().unwrap();
    let fd = sock.fd();
    let sock2 = Socket::from_fd(fd).unwrap();
    assert_eq!(sock2.fd(), fd);
}

#[test]
fn test_accept_connection_would_block() {
    let sock = Socket::new().unwrap();
    sock.set_reuseaddr().unwrap();
    sock.bind("127.0.0.1", 0).unwrap();
    sock.listen(128).unwrap();
    sock.set_nonblocking().unwrap();

    let result = ConnectionManager::accept_connection(&sock);
    assert!(result.is_err());
}

#[test]
fn test_parse_epoll_event_read() {
    let events = parse_epoll_event(5, EPOLLIN as u32);
    assert_eq!(events.len(), 1);
    assert!(matches!(events[0], Event::Read(5)));
}

#[test]
fn test_parse_epoll_event_multiple() {
    let events = parse_epoll_event(5, EPOLLIN as u32 | libc::EPOLLOUT as u32);
    assert_eq!(events.len(), 2);
    let mut has_read = false;
    let mut has_write = false;
    for event in &events {
        if *event == Event::Read(5) {
            has_read = true;
        }
        if *event == Event::Write(5) {
            has_write = true;
        }
    }
    assert!(has_read);
    assert!(has_write);
}

#[test]
fn test_parse_epoll_event_hup() {
    let events = parse_epoll_event(5, libc::EPOLLHUP as u32);
    assert_eq!(events.len(), 1);
    assert!(matches!(events[0], Event::HangUp(5)));
}

#[test]
fn test_parse_epoll_event_error() {
    let events = parse_epoll_event(5, libc::EPOLLERR as u32);
    assert_eq!(events.len(), 1);
    assert!(matches!(events[0], Event::HangUp(5)));
}

#[test]
fn test_parse_epoll_event_empty() {
    let events = parse_epoll_event(5, 0);
    assert_eq!(events.len(), 0);
}
