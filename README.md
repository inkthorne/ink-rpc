# ink-rpc

A Rust library for creating and handling JSON-RPC 2.0 requests and responses.

[![Crates.io](https://img.shields.io/crates/v/ink-rpc.svg)](https://crates.io/crates/ink-rpc)
[![Documentation](https://docs.rs/ink-rpc/badge.svg)](https://docs.rs/ink-rpc)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/inkthorne/ink-rpc)

## Overview

`ink-rpc` provides a simple and type-safe way to work with JSON-RPC 2.0 protocol in Rust. It offers structures for creating requests and responses with automatic ID management, parameter handling, and JSON serialization/deserialization.

## Features

- ✅ **JSON-RPC 2.0 Compliant** - Follows the JSON-RPC 2.0 specification
- ✅ **Automatic ID Management** - Auto-incrementing request IDs with thread-safe atomic counters
- ✅ **Builder Pattern** - Fluent API for constructing requests and responses
- ✅ **Type Safety** - Strongly typed structures with serde integration
- ✅ **Error Handling** - Support for both successful results and error responses
- ✅ **Batch Operations** - Handle multiple requests and responses
- ✅ **Pretty Printing** - Human-readable JSON output via Display trait
- ✅ **Zero Dependencies** - Only depends on `serde` and `serde_json`

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ink-rpc = "0.1.0"
```

## Quick Start

### Creating a Request

```rust
use ink_rpc::{RpcRequest, RpcResponse};
use serde_json::json;

// Simple request with no parameters
let mut request = RpcRequest::new();
request.set_method("get_server_info".to_string());

// Request with parameters
let mut transfer_request = RpcRequest::new();
transfer_request
    .set_method("transfer_funds".to_string())
    .set_params(json!({
        "from_account": "acc_123456",
        "to_account": "acc_789012",
        "amount": 250.75,
        "currency": "USD"
    }));

println!("{}", transfer_request); // Pretty-printed JSON
```

### Creating a Response

```rust
use ink_rpc::RpcResponse;
use serde_json::json;

// Successful response
let mut response = RpcResponse::new(request.id());
response.set_result(json!({
    "transaction_id": "txn_abc123def456",
    "status": "completed",
    "timestamp": "2025-06-22T10:30:00Z"
}));

// Error response
let mut error_response = RpcResponse::new(request.id());
error_response.set_error(json!({
    "code": -32001,
    "message": "Insufficient funds",
    "data": {
        "requested_amount": 5000.0,
        "available_balance": 1749.25
    }
}));
```

## API Reference

### RpcRequest

The `RpcRequest` struct represents a JSON-RPC 2.0 request message.

#### Methods

- `new()` - Creates a new request with auto-generated ID
- `set_method(method: String)` - Sets the RPC method name
- `set_params(params: JsonValue)` - Sets the request parameters
- `id()` - Returns the request ID
- `method()` - Returns the method name
- `params()` - Returns the parameters
- `to_json()` - Converts to JSON value
- `from_json(json: JsonValue)` - Creates from JSON value

### RpcResponse

The `RpcResponse` struct represents a JSON-RPC 2.0 response message.

#### Methods

- `new(id: u64)` - Creates a new response for the given request ID
- `set_result(result: JsonValue)` - Sets successful result data
- `set_error(error: JsonValue)` - Sets error information
- `id()` - Returns the response ID
- `result()` - Returns the result (if present)
- `error()` - Returns the error (if present)
- `is_success()` - Checks if response contains a result
- `is_error()` - Checks if response contains an error
- `from_json(json: JsonValue)` - Creates from JSON value

## Examples

### Basic Usage

```rust
use ink_rpc::{RpcRequest, RpcResponse};
use serde_json::json;

fn main() {
    // Create a request
    let mut request = RpcRequest::new();
    request
        .set_method("get_balance".to_string())
        .set_params(json!({"account": "acc_123456"}));

    println!("Request: {}", request);

    // Create a corresponding response
    let mut response = RpcResponse::new(request.id());
    response.set_result(json!({
        "balance": 1749.25,
        "currency": "USD"
    }));

    println!("Response: {}", response);
}
```

### Error Handling

```rust
use ink_rpc::{RpcRequest, RpcResponse};
use serde_json::json;

let mut request = RpcRequest::new();
request.set_method("invalid_method".to_string());

let mut error_response = RpcResponse::new(request.id());
error_response.set_error(json!({
    "code": -32601,
    "message": "Method not found",
    "data": {
        "method": "invalid_method"
    }
}));

assert!(error_response.is_error());
assert!(!error_response.is_success());
```

### Batch Processing

```rust
use ink_rpc::{RpcRequest, RpcResponse};
use serde_json::json;

// Create multiple requests
let requests = vec![
    {
        let mut req = RpcRequest::new();
        req.set_method("get_balance".to_string())
           .set_params(json!({"account": "acc_123456"}));
        req
    },
    {
        let mut req = RpcRequest::new();
        req.set_method("get_transactions".to_string())
           .set_params(json!({"account": "acc_123456", "limit": 10}));
        req
    }
];

// Process and create responses
let responses: Vec<RpcResponse> = requests
    .iter()
    .map(|req| {
        let mut resp = RpcResponse::new(req.id());
        // Process the request and set result/error
        resp.set_result(json!({"status": "processed"}));
        resp
    })
    .collect();
```

## JSON-RPC 2.0 Specification

This library follows the [JSON-RPC 2.0 specification](https://www.jsonrpc.org/specification). 

### Request Format

```json
{
    "jsonrpc": "2.0",
    "method": "method_name",
    "params": { "param1": "value1" },
    "id": 1
}
```

### Response Format

**Success:**
```json
{
    "jsonrpc": "2.0",
    "result": { "data": "value" },
    "id": 1
}
```

**Error:**
```json
{
    "jsonrpc": "2.0",
    "error": {
        "code": -32001,
        "message": "Error description",
        "data": { "additional": "info" }
    },
    "id": 1
}
```

## Running Examples

The `examples/` directory contains comprehensive usage examples:

```bash
# Run the basic usage example
cargo run --example basic_usage
```

This will demonstrate:
- Simple requests with null parameters
- Complex requests with JSON parameters  
- Successful responses with result data
- Error responses with error details
- Batch request simulation
- Pretty-printed JSON output

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Changelog

### v0.1.0 (2025-06-24)

- Initial release
- JSON-RPC 2.0 request and response structures
- Automatic ID management
- Builder pattern API
- Pretty-printing support
- Comprehensive examples
