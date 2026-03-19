# sync-bfu

A Rust-based S3 file synchronization tool.

## Features

- Secure S3 file synchronization
- Proper error handling and logging
- Configuration via environment variables
- Async/await support for high performance
- Type-safe operations with comprehensive error handling

## Environment Variables

Configure the application using the following environment variables:

- `SYNC_BFU_BUCKET`: S3 bucket name (required)
- `SYNC_BFU_REGION`: AWS region (required)
- `SYNC_BFU_ACCESS_KEY`: AWS access key (required)
- `SYNC_BFU_SECRET_KEY`: AWS secret key (required)
- `SYNC_BFU_ENDPOINT`: Custom S3 endpoint (optional)

## Installation

```bash
cargo build --release
```

## Usage

```bash
export SYNC_BFU_BUCKET="your-bucket-name"
export SYNC_BFU_REGION="us-east-1"
export SYNC_BFU_ACCESS_KEY="your-access-key"
export SYNC_BFU_SECRET_KEY="your-secret-key"

cargo run
```

## Development

This project uses Rust 2024 edition with modern async patterns and comprehensive error handling.

### Project Structure

- `src/main.rs`: Application entry point
- `src/config.rs`: Configuration management
- `src/error.rs`: Custom error types
- `src/manager/`: Core file management logic
  - `data_manager.rs`: S3 operations
  - `file_manager.rs`: Local file management

## License

MIT
