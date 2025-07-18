# EVM Compatible JSON RPC Server for Shardeum

A high-performance JSON-RPC server implementation for Shardeum-related EVM blockchain interactions, built with Rust and love.

## 🚀 Features

- **Fast & Reliable**: Built with Rust for optimal performance and memory safety
- **JSON-RPC 2.0**: Full compliance with JSON-RPC 2.0 specification
- **Configurable**: TOML-based configuration with sensible defaults
- **Asynchronous**: Non-blocking I/O using Tokio runtime
- **Extensible**: Modular architecture for easy method additions
- **CI/CD Ready**: GitHub Actions workflow for automated testing

## 📋 Prerequisites

- Rust 1.70+ (2021 edition)
- Cargo package manager

## 🛠️ Installation

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd rust_rpc
   ```

2. **Build the project**
   ```bash
   cargo build --release
   ```

3. **Run the server**
   ```bash
   cargo run
   ```

## ⚙️ Configuration

The server uses a TOML configuration file located at `src/config.toml`. Customize these settings:

```toml
# Server settings
host = "localhost"
port = 8080
request_timeout = 30  # in seconds
verbose = true

# List of node URLs
node_urls = [
    "http://localhost:3000"
]
```

### Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `host` | String | `"localhost"` | Server bind address |
| `port` | Integer | `8080` | Server port |
| `request_timeout` | Integer | `30` | Request timeout in seconds |
| `verbose` | Boolean | `false` | Enable verbose logging |
| `node_urls` | Array | `["http://localhost:3000"]` | List of Network node URLs |

## 🔌 API Usage

The server accepts JSON-RPC 2.0 requests via HTTP POST to the root endpoint `/`.

### Request Format

```json
{
  "jsonrpc": "2.0",
  "method": "method_name",
  "params": {},
  "id": 1
}
```

### Response Format

```json
{
  "jsonrpc": "2.0",
  "result": "response_data",
  "error": null,
  "id": 1
}
```

### Available Methods

- `dummy` - Test method that returns the method name

### Example Request

```bash
curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "dummy",
    "params": {},
    "id": 1
  }'
```

## 🏗️ Project Structure

```
src/
├── main.rs          # Application entry point and server setup
├── api.rs           # JSON-RPC request/response structures
├── config.rs        # Configuration management
├── methods.rs       # RPC method implementations
├── shardeum.rs      # Shardeum-specific functionality
├── middleware.rs    # HTTP middleware (future use)
├── utils.rs         # Utility functions (future use)
└── config.toml      # Configuration file
```

## 🧪 Testing

Run the test suite:

```bash
cargo test --verbose
```

Check code formatting:

```bash
cargo fmt --check
```

## 🚀 Development

### Adding New RPC Methods

1. **Implement the method** in `src/methods.rs`:
   ```rust
   pub fn your_method(payload: RpcRequest) -> RpcResponse {
       // Your implementation here
   }
   ```

2. **Register the method** in `src/main.rs`:
   ```rust
   match method.as_str() {
       "dummy" => methods::lib_dummy(payload),
       "your_method" => methods::your_method(payload),
       // ...
   }
   ```

### Code Style

This project follows standard Rust formatting. Run `cargo fmt` before committing.

## 📦 Dependencies

- **axum** (0.8.4) - Web framework
- **tokio** (1.46.1) - Async runtime  
- **reqwest** (0.12.22) - HTTP client
- **serde** (1.0.219) - Serialization framework
- **tracing** (0.1.41) - Logging framework
- **toml** (0.8) - Configuration parsing

## 🔄 CI/CD

The project includes a GitHub Actions workflow that:
- Checks code formatting
- Builds the project
- Runs all tests

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

If you encounter any issues or have questions:

1. Check the [Issues](../../issues) section
2. Create a new issue with detailed information
3. Include relevant logs and configuration details

---

**Built with ❤️ and Rust**
