use serde::{Deserialize, Serialize};
use serde_json::Error as JsonError;
use serde_json::Value as JsonValue;
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

// ===========================================================================
// STRUCT: RpcRequest
// ===

#[derive(Serialize, Deserialize, Clone)]
pub struct RpcRequest {
    jsonrpc: String,
    method: String,
    params: JsonValue,
    id: u64,
}

impl RpcRequest {
    /// Creates a new RPC request with default values.
    ///
    /// The request is initialized with:
    /// - `jsonrpc`: "2.0" (JSON-RPC protocol version)
    /// - `method`: Empty string (to be set later)
    /// - `params`: JSON null value
    /// - `id`: Auto-incremented unique identifier
    ///    /// # Examples
    ///
    /// ```
    /// use ink_rpc::request::RpcRequest;
    /// let request = RpcRequest::new();
    /// assert_eq!(request.method(), "");
    /// ```
    pub fn new() -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            method: String::new(),
            params: JsonValue::Null,
            id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
        }
    }

    /// Converts the RPC request to a JSON value.
    ///
    /// Serializes the entire RpcRequest struct into a serde_json::Value.
    /// If serialization fails, returns `JsonValue::Null`.
    ///
    /// # Returns
    ///
    /// A `JsonValue` representation of the request, or `JsonValue::Null` on error.
    ///    /// # Examples
    ///
    /// ```
    /// use ink_rpc::request::RpcRequest;
    /// let mut request = RpcRequest::new();
    /// request.set_method("get_balance".to_string());
    /// let json = request.to_json();
    /// ```
    pub fn to_json(&self) -> JsonValue {
        serde_json::to_value(self).unwrap_or_else(|_| JsonValue::Null)
    }

    /// Creates an RPC request from a JSON value.
    ///
    /// Deserializes a JSON value into an RpcRequest struct. The JSON must contain
    /// the required fields: `jsonrpc`, `method`, `params`, and `id`.
    ///
    /// # Arguments
    ///
    /// * `json` - A JSON value containing the request data
    ///
    /// # Returns
    ///
    /// * `Ok(RpcRequest)` - Successfully deserialized request
    /// * `Err(JsonError)` - Deserialization failed
    ///    /// # Examples
    ///
    /// ```
    /// use ink_rpc::request::RpcRequest;
    /// let json = serde_json::json!({
    ///     "jsonrpc": "2.0",
    ///     "method": "get_balance",
    ///     "params": null,
    ///     "id": 1
    /// });
    /// let request = RpcRequest::from_json(json).unwrap();
    /// ```
    pub fn from_json(json: JsonValue) -> Result<Self, JsonError> {
        serde_json::from_value(json)
    }

    /// Returns the unique identifier of this RPC request.
    ///
    /// Each request is assigned a unique ID when created using `new()`.
    /// This ID is used to match requests with their corresponding responses.
    ///
    /// # Returns
    ///
    /// The request's unique identifier as a `u64`.
    ///    /// # Examples
    ///
    /// ```
    /// use ink_rpc::request::RpcRequest;
    /// let request = RpcRequest::new();
    /// let id = request.id();
    /// assert!(id > 0);
    /// ```
    // Accessor for id
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Returns the method name of this RPC request.
    ///
    /// The method specifies which remote procedure should be called.
    ///
    /// # Returns
    ///
    /// A string slice containing the method name.
    ///    /// # Examples
    ///
    /// ```
    /// use ink_rpc::request::RpcRequest;
    /// let mut request = RpcRequest::new();
    /// request.set_method("get_balance".to_string());
    /// assert_eq!(request.method(), "get_balance");
    /// ```
    // Accessor for method
    pub fn method(&self) -> &str {
        &self.method
    }

    /// Sets the method name for this RPC request.
    ///
    /// The method specifies which remote procedure should be called.
    /// This function uses the builder pattern, returning a mutable reference
    /// to self for method chaining.
    ///
    /// # Arguments
    ///
    /// * `method` - The name of the method to call
    ///
    /// # Returns
    ///
    /// A mutable reference to self, allowing for method chaining.
    ///    /// # Examples
    ///
    /// ```
    /// use ink_rpc::request::RpcRequest;
    /// let mut request = RpcRequest::new();
    /// request.set_method("get_balance".to_string())
    ///        .set_params(serde_json::json!({"account": "123"}));
    /// ```
    // Mutator for method
    pub fn set_method(&mut self, method: String) -> &mut Self {
        self.method = method;
        self
    }

    /// Returns the parameters of this RPC request.
    ///
    /// Parameters contain the arguments to be passed to the remote method.
    /// They can be any valid JSON value (object, array, primitive, or null).
    ///
    /// # Returns
    ///
    /// A reference to the JSON value containing the parameters.
    ///    /// # Examples
    ///
    /// ```
    /// use ink_rpc::request::RpcRequest;
    /// let mut request = RpcRequest::new();
    /// let params = serde_json::json!({"account": "123", "amount": 100});
    /// request.set_params(params.clone());
    /// assert_eq!(request.params(), &params);
    /// ```
    // Accessor for params
    pub fn params(&self) -> &JsonValue {
        &self.params
    }

    /// Sets the parameters for this RPC request.
    ///
    /// Parameters contain the arguments to be passed to the remote method.
    /// They can be any valid JSON value (object, array, primitive, or null).
    /// This function uses the builder pattern, returning a mutable reference
    /// to self for method chaining.
    ///
    /// # Arguments
    ///
    /// * `params` - The JSON value containing the method parameters
    ///
    /// # Returns
    ///
    /// A mutable reference to self, allowing for method chaining.
    ///    /// # Examples
    ///
    /// ```
    /// use ink_rpc::request::RpcRequest;
    /// let mut request = RpcRequest::new();
    /// let params = serde_json::json!({"account": "123", "amount": 100});
    /// request.set_method("transfer".to_string())
    ///        .set_params(params);
    /// ```
    // Mutator for params
    pub fn set_params(&mut self, params: JsonValue) -> &mut Self {
        self.params = params;
        self
    }
}

impl fmt::Display for RpcRequest {
    /// Formats the RPC request as a pretty-printed JSON string.
    ///
    /// This implementation of the `Display` trait converts the RPC request
    /// to its JSON representation using `to_json()` and then formats it
    /// as a pretty-printed JSON string for human-readable output.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to write the output to
    ///
    /// # Returns
    ///
    /// A `fmt::Result` indicating success or failure of the formatting operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use ink_rpc::request::RpcRequest;
    /// let mut request = RpcRequest::new();
    /// request.set_method("get_balance".to_string());
    /// println!("{}", request); // Prints pretty-formatted JSON
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = self.to_json();
        let json_string = serde_json::to_string_pretty(&json).unwrap_or("{}".to_string());

        write!(f, "{}", &json_string)
    }
}

// ===========================================================================
// TESTS: RpcRequest
// ===

#[cfg(test)]
mod tests {
    use super::*;

    // ---------------------------------------------------------------------------
    // Constructor and Core Functionality Tests
    // ---------------------------------------------------------------------------

    #[test]
    fn test_new_request_initialization() {
        let request1 = RpcRequest::new();
        let request2 = RpcRequest::new();

        // Test default values
        assert_eq!(request1.method(), "");
        assert_eq!(request1.params(), &JsonValue::Null);

        // Test unique ID generation
        assert_ne!(request1.id(), request2.id());
        assert!(request1.id() > 0);
        assert!(request2.id() > 0);
    }

    // ---------------------------------------------------------------------------
    // Method Field Tests
    // ---------------------------------------------------------------------------

    #[test]
    fn test_method_operations() {
        let mut request = RpcRequest::new();

        // Test initial state
        assert_eq!(request.method(), "");

        // Test setting and getting
        request.set_method("get_balance".to_string());
        assert_eq!(request.method(), "get_balance");

        // Test overwriting
        request.set_method("transfer".to_string());
        assert_eq!(request.method(), "transfer");

        // Test empty string
        request.set_method("".to_string());
        assert_eq!(request.method(), "");

        // Test method chaining works
        request
            .set_method("chained_method".to_string())
            .set_method("final_method".to_string());
        assert_eq!(request.method(), "final_method");
    }

    // ---------------------------------------------------------------------------
    // Params Field Tests
    // ---------------------------------------------------------------------------

    #[test]
    fn test_params_operations() {
        let mut request = RpcRequest::new();

        // Test initial state
        assert_eq!(request.params(), &JsonValue::Null);

        // Test setting simple params
        let simple_params = serde_json::json!({"amount": 100});
        request.set_params(simple_params.clone());
        assert_eq!(request.params(), &simple_params);

        // Test overwriting params
        let new_params = serde_json::json!({"to": "address123"});
        request.set_params(new_params.clone());
        assert_eq!(request.params(), &new_params);

        // Test setting back to null
        request.set_params(JsonValue::Null);
        assert_eq!(request.params(), &JsonValue::Null);

        // Test method chaining works
        request
            .set_params(simple_params.clone())
            .set_params(new_params.clone());
        assert_eq!(request.params(), &new_params);
    }

    #[test]
    fn test_params_complex_structures() {
        let mut request = RpcRequest::new();

        // Test complex nested structure
        let complex_params = serde_json::json!({
            "transaction": {
                "from": "addr1",
                "to": "addr2",
                "amount": 150.75,
                "metadata": {
                    "timestamp": 1234567890,
                    "nonce": 42
                }
            },
            "options": ["fast", "secure"]
        });

        request.set_params(complex_params.clone());
        assert_eq!(request.params(), &complex_params);

        // Verify nested access works
        let params = request.params();
        assert_eq!(params["transaction"]["amount"], 150.75);
        assert_eq!(params["options"][0], "fast");
    }

    // ---------------------------------------------------------------------------
    // JSON Serialization Tests
    // ---------------------------------------------------------------------------

    #[test]
    fn test_json_serialization() {
        let mut request = RpcRequest::new();
        request
            .set_method("test_method".to_string())
            .set_params(serde_json::json!({"key": "value"}));

        // Test to_json
        let json = request.to_json();
        assert!(json.is_object());
        assert_eq!(json["jsonrpc"], "2.0");
        assert_eq!(json["method"], "test_method");
        assert_eq!(json["params"]["key"], "value");
        assert_eq!(json["id"], request.id());

        // Test roundtrip
        let deserialized = RpcRequest::from_json(json).expect("Failed to deserialize");
        assert_eq!(request.method(), deserialized.method());
        assert_eq!(request.params(), deserialized.params());
        assert_eq!(request.id(), deserialized.id());
    }

    #[test]
    fn test_json_deserialization() {
        // Test valid JSON
        let valid_json = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "test_method",
            "params": {"account": "123"},
            "id": 42
        });

        let request = RpcRequest::from_json(valid_json).expect("Failed to deserialize");
        assert_eq!(request.method(), "test_method");
        assert_eq!(request.params()["account"], "123");
        assert_eq!(request.id(), 42);

        // Test invalid JSON
        let invalid_json = serde_json::json!({"invalid": "structure"});
        let result = RpcRequest::from_json(invalid_json);
        assert!(result.is_err());
    }

    // ---------------------------------------------------------------------------
    // Display Trait Tests
    // ---------------------------------------------------------------------------

    #[test]
    fn test_display_formatting() {
        let mut request = RpcRequest::new();
        request
            .set_method("get_balance".to_string())
            .set_params(serde_json::json!({"account": "test"}));

        let display_string = format!("{}", request);

        // Verify JSON structure and content
        assert!(display_string.starts_with('{'));
        assert!(display_string.ends_with('}'));
        assert!(display_string.contains("\"jsonrpc\": \"2.0\""));
        assert!(display_string.contains("\"method\": \"get_balance\""));
        assert!(display_string.contains("\"account\": \"test\""));
        assert!(display_string.contains(&format!("\"id\": {}", request.id())));

        // Verify it's valid JSON by parsing it back
        let parsed: serde_json::Value =
            serde_json::from_str(&display_string).expect("Display output should be valid JSON");
        assert_eq!(parsed["method"], "get_balance");
    }

    // ---------------------------------------------------------------------------
    // Builder Pattern Tests
    // ---------------------------------------------------------------------------

    #[test]
    fn test_builder_pattern() {
        let mut request = RpcRequest::new();

        // Test method chaining
        request
            .set_method("transfer".to_string())
            .set_params(serde_json::json!({
                "from": "addr1",
                "to": "addr2",
                "amount": 100
            }));

        assert_eq!(request.method(), "transfer");
        assert_eq!(request.params()["amount"], 100);
        assert_eq!(request.params()["from"], "addr1");
    }
}
