# Examples

This directory contains example code demonstrating how to use the `ink-rpc` library.

## Running Examples

To run an example, use the following command from the project root:

```bash
cargo run --example <example_name>
```

## Available Examples

### `basic_usage.rs`

Demonstrates the fundamental usage of `RpcRequest` and `RpcResponse`:

- Creating simple requests with null parameters
- Creating requests with complex JSON parameters
- Generating successful responses with result data
- Generating error responses with error details
- Simulating batch requests and responses
- Pretty-printing JSON-RPC messages to stdout

Run this example:
```bash
cargo run --example basic_usage
```

This example showcases:
- ✅ Request creation with auto-incrementing IDs
- ✅ Method and parameter setting using builder pattern
- ✅ Response creation with matching request IDs
- ✅ Setting result data in responses
- ✅ Setting error information in responses
- ✅ Pretty-formatted JSON output via the `Display` trait
- ✅ Real-world scenarios like financial transactions

The output demonstrates proper JSON-RPC 2.0 formatting and shows how request IDs are automatically matched between requests and responses.
