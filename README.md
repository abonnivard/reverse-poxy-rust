# Reverse Proxy in Rust

This project implements a reverse proxy in Rust, similar to Nginx, using `hyper` and `tokio` for asynchronous handling of HTTP requests.

## Features

- Reverse proxy functionality
- Asynchronous request handling

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/) must be installed on your system. If it's not installed, you can install it by running:

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

### Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/abonnivard/reverse-rpoxy-rust.git
    cd reverse-proxy-rust
    ```

2. Build the project:

    ```bash
    make build
    ```

3. Run the server:

    ```bash
    make run
    ```

By default, the proxy server listens on `http://127.0.0.1:3000`. You can modify this in the source code if needed.

### Running Tests

You can run unit tests using the following command:

```bash
make test
