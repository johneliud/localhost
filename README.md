# Localhost HTTP Server

A lightweight, non-blocking HTTP server written in Rust using epoll.

## Features

- Non-blocking TCP socket using epoll
- Configurable host and port
- Modular architecture (socket, epoll, config modules)
- Comprehensive test suite

## Requirements

- Rust (latest stable)
- Linux (epoll is Linux-specific)

## Installation

### Clone the repository

```bash
git clone https://github.com/johneliud/localhost.git
cd localhost
```

### Build

```bash
cargo build --release
```

## Usage

Run the server with default settings (0.0.0.0:8080):

```bash
cargo run
```

Or specify a custom port:

```bash
cargo run -- --port 9000
```

### Command-line Options

| Option | Description | Default |
|--------|-------------|---------|
| `--port` | Port to listen on | `8080` |
| `--host` | Host to bind to | `0.0.0.0` |

## Testing

Run all tests:

```bash
cargo test
```

Run specific test suites:

```bash
cargo test --test socket_test
cargo test --test epoll_test
cargo test --test config_test
```

Run with output:

```bash
cargo test -- --nocapture
```

## Linting

```bash
cargo clippy
```

## Project Structure

```
src/
├── lib.rs          # Library root
├── main.rs         # Binary entry point
├── socket.rs       # Socket handling
├── epoll.rs        # Epoll event loop
└── config.rs       # Configuration

tests/
├── socket_test.rs  # Socket tests
├── epoll_test.rs  # Epoll tests
└── config_test.rs # Config tests
```

## License

MIT
