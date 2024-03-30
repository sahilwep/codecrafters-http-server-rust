# CodeCrafters HTTP Server in Rust

This is a simple HTTP server implemented in Rust. It's designed to handle basic `HTTP methods` like `GET`, `POST`, `PUT`, and `DELETE`. The server can serve static files, handle simple routing, and demonstrate basic concurrency using Rust's threading capabilities.

## Features

- Handles basic HTTP methods: `GET`, `POST`, `PUT`, `DELETE`
- Serves static files
- Simple routing functionality
- Demonstrates concurrency using threads

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/sahilwep/codecrafters-http-server-rust.git
   ```

2. Navigate to the project directory:

   ```bash
   cd codecrafters-http-server-rust
   ```

3. Build the project:

   ```bash
   cargo build
   ```

4. Run the server:

   ```bash
   cargo run
   ```

## Usage

Once the server is running, you can access it by navigating to `http://localhost:4221` in your web browser. Here are some example routes:

- `/`: Returns a simple "Hello World" message.
- `/user-agent`: Returns the user-agent string of the client.
- `/files/{filename}`: Serves the specified file from the server's directory.
- `/echo/{text}`: Echoes back the provided text.

## Configuration

You can configure the server by modifying the source code directly. The main configuration options include:

- Specifying the port the server listens on (`main()` function).
- Setting the server's root directory for serving files (`parse_args()` function).

## Dependencies

- `tokio`: Asynchronous runtime for Rust.
- `bytes`: Utilities for working with bytes.
- `serde_json`: JSON serialization and deserialization.


***