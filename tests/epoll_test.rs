use libc::{epoll_event, EPOLLIN};
use localhost::{epoll::Epoll, socket::Socket};

#[test]
fn test_epoll_create() {
    let epoll = Epoll::new();
    assert!(epoll.is_ok());
    let epoll = epoll.unwrap();
    assert!(epoll.fd() >= 0);
}

#[test]
fn test_epoll_add_read() {
    let mut epoll = Epoll::new().unwrap();
    let sock = Socket::new().unwrap();

    let result = epoll.add_read(sock.fd());
    assert!(result.is_ok());
}

#[test]
fn test_epoll_ctl_add() {
    let epoll = Epoll::new().unwrap();
    let sock = Socket::new().unwrap();

    let mut event = epoll_event {
        events: EPOLLIN as u32,
        u64: sock.fd() as u64,
    };

    let result = unsafe { libc::epoll_ctl(epoll.fd(), libc::EPOLL_CTL_ADD, sock.fd(), &mut event) };
    assert_eq!(result, 0, "Adding fd to epoll should succeed");
}

#[test]
fn test_multiple_sockets_same_epoll() {
    let epoll = Epoll::new().unwrap();

    let sock1 = Socket::new().unwrap();
    let sock2 = Socket::new().unwrap();

    let mut event1 = epoll_event {
        events: EPOLLIN as u32,
        u64: sock1.fd() as u64,
    };
    let mut event2 = epoll_event {
        events: EPOLLIN as u32,
        u64: sock2.fd() as u64,
    };

    assert_eq!(
        unsafe { libc::epoll_ctl(epoll.fd(), libc::EPOLL_CTL_ADD, sock1.fd(), &mut event1) },
        0
    );
    assert_eq!(
        unsafe { libc::epoll_ctl(epoll.fd(), libc::EPOLL_CTL_ADD, sock2.fd(), &mut event2) },
        0
    );
}

#[test]
fn test_epoll_wait_no_events() {
    let epoll = Epoll::new().unwrap();

    let result = epoll.wait(0);
    assert!(result.is_ok());
    let events = result.unwrap();
    assert_eq!(events.len(), 0);
}

#[test]
fn test_epoll_remove() {
    let mut epoll = Epoll::new().unwrap();
    let sock = Socket::new().unwrap();

    epoll.add_read(sock.fd()).unwrap();
    let result = epoll.remove(sock.fd());
    assert!(result.is_ok());
}
