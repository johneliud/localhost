use libc::{c_int, epoll_event, EPOLLIN};
use std::collections::HashMap;

/**
 * Maximum number of events that can be returned by epoll_wait.
 */
pub const MAX_EVENTS: usize = 1024;

/**
 * Represents an epoll instance for multiplexing I/O events.
 */
pub struct Epoll {
    fd: c_int,
    registered_fds: HashMap<c_int, u32>,
}

impl Epoll {
    /**
     * Creates a new epoll instance.
     *
     * Returns Ok(Epoll) on success or Err(EpollError) on failure.
     */
    pub fn new() -> Result<Self, EpollError> {
        unsafe {
            let fd = libc::epoll_create1(0);
            if fd < 0 {
                return Err(EpollError::Create);
            }
            Ok(Self {
                fd,
                registered_fds: HashMap::new(),
            })
        }
    }

    /**
     * Adds a file descriptor to the epoll instance with the specified events.
     *
     * # Arguments
     * * `fd` - The file descriptor to monitor
     * * `events` - The events to monitor (e.g., EPOLLIN for read readiness)
     *
     * Returns Ok(()) on success or Err(EpollError) on failure.
     */
    pub fn add(&mut self, fd: c_int, events: u32) -> Result<(), EpollError> {
        let mut event = epoll_event {
            events,
            u64: fd as u64,
        };

        unsafe {
            if libc::epoll_ctl(self.fd, libc::EPOLL_CTL_ADD, fd, &mut event) < 0 {
                return Err(EpollError::CtlAdd);
            }
        }

        self.registered_fds.insert(fd, events);
        Ok(())
    }

    /**
     * Adds a file descriptor to monitor for read readiness (EPOLLIN).
     *
     * # Arguments
     * * `fd` - The file descriptor to monitor
     *
     * Returns Ok(()) on success or Err(EpollError) on failure.
     */
    pub fn add_read(&mut self, fd: c_int) -> Result<(), EpollError> {
        self.add(fd, EPOLLIN as u32)
    }

    /**
     * Removes a file descriptor from the epoll instance.
     *
     * # Arguments
     * * `fd` - The file descriptor to remove
     *
     * Returns Ok(()) on success or Err(EpollError) on failure.
     */
    pub fn remove(&mut self, fd: c_int) -> Result<(), EpollError> {
        let mut empty_event = epoll_event {
            events: 0,
            u64: fd as u64,
        };

        unsafe {
            if libc::epoll_ctl(self.fd, libc::EPOLL_CTL_DEL, fd, &mut empty_event) < 0 {
                return Err(EpollError::CtlRemove);
            }
        }

        self.registered_fds.remove(&fd);
        Ok(())
    }

    /**
     * Waits for events to become ready on the monitored file descriptors.
     *
     * # Arguments
     * * `timeout_ms` - Timeout in milliseconds (-1 for blocking, 0 for non-blocking)
     *
     * Returns Ok(Vec<EpollEvent>) on success or Err(EpollError) on failure.
     */
    pub fn wait(&self, timeout_ms: i32) -> Result<Vec<EpollEvent>, EpollError> {
        let mut events = vec![EpollEvent::default(); MAX_EVENTS];

        let n = unsafe {
            libc::epoll_wait(
                self.fd,
                events.as_mut_ptr() as *mut epoll_event,
                MAX_EVENTS as c_int,
                timeout_ms,
            )
        };

        if n < 0 {
            return Err(EpollError::Wait);
        }

        unsafe { events.set_len(n as usize) };
        Ok(events)
    }

    /**
     * Returns the raw file descriptor for the epoll instance.
     */
    pub fn fd(&self) -> c_int {
        self.fd
    }

    /**
     * Closes the epoll file descriptor.
     */
    pub fn close(&self) {
        unsafe {
            libc::close(self.fd);
        }
    }
}

impl Drop for Epoll {
    /**
     * Automatically closes the epoll instance when it goes out of scope.
     */
    fn drop(&mut self) {
        self.close();
    }
}

/**
 * Represents an event returned by epoll_wait.
 */
#[derive(Debug, Clone, Copy, Default)]
pub struct EpollEvent {
    /** The events that triggered this event. */
    pub events: u32,
    /** The file descriptor associated with this event. */
    pub fd: c_int,
}

impl From<epoll_event> for EpollEvent {
    /**
     * Converts from the raw libc epoll_event structure.
     */
    fn from(e: epoll_event) -> Self {
        Self {
            events: e.events,
            fd: e.u64 as c_int,
        }
    }
}

/**
 * Error types that can occur when working with epoll.
 */
#[derive(Debug, Clone, Copy)]
pub enum EpollError {
    Create,
    CtlAdd,
    CtlRemove,
    Wait,
}

impl std::fmt::Display for EpollError {
    /**
     * Formats the error for display purposes.
     */
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EpollError::Create => write!(f, "Failed to create epoll instance"),
            EpollError::CtlAdd => write!(f, "Failed to add fd to epoll"),
            EpollError::CtlRemove => write!(f, "Failed to remove fd from epoll"),
            EpollError::Wait => write!(f, "epoll_wait failed"),
        }
    }
}

impl std::error::Error for EpollError {}
