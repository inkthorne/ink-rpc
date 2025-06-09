use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use serde_json::json;
use std::fmt;

// ===========================================================================
// STRUCT: RpcResponse
// ===

/// Represents a JSON-RPC 2.0 response message.
///
/// This struct wraps a JSON value that contains the response data according to the
/// JSON-RPC 2.0 specification. It provides methods to create, manipulate, and access
/// the response data including ID, result, and error information.
#[derive(Serialize, Deserialize, Clone)]
pub struct RpcResponse {
    value: JsonValue,
}

impl RpcResponse {
    /// Creates a new RPC response with the specified ID.
    ///
    /// This creates a minimal JSON-RPC 2.0 response containing only the protocol version
    /// and ID. The result field should be set later using `set_result()`.
    ///
    /// # Arguments
    ///
    /// * `id` - The request ID that this response corresponds to
    ///
    /// # Examples
    ///
    /// ```
    /// use ink_rpc::RpcResponse;
    /// let response = RpcResponse::new(123);
    /// assert_eq!(response.id(), 123);
    /// ```
    pub fn new(id: u64) -> Self {
        let value = json!(
            {
                "jsonrpc": "2.0",
                "id": id,
            }
        );

        RpcResponse { value }
    }

    /// Creates an RPC response from an existing JSON value.
    ///
    /// This constructor allows creating a response from a pre-built JSON object,
    /// useful when parsing responses from external sources or when you have
    /// already constructed the response JSON.
    ///
    /// # Arguments
    ///
    /// * `value` - A JSON value representing the complete RPC response
    ///
    /// # Examples
    ///
    /// ```
    /// use ink_rpc::RpcResponse;
    /// use serde_json::json;
    ///
    /// let json_response = json!({
    ///     "jsonrpc": "2.0",
    ///     "id": 1,
    ///     "result": "success"
    /// });
    /// let response = RpcResponse::from_json(json_response);
    /// ```
    pub fn from_json(value: JsonValue) -> Self {
        RpcResponse { value }
    }

    /// Returns a reference to the underlying JSON value.
    ///
    /// This provides direct access to the raw JSON representation of the response,
    /// allowing for custom manipulation or serialization.
    ///
    /// # Returns
    ///
    /// A reference to the internal `JsonValue`
    pub fn as_json(&self) -> &JsonValue {
        &self.value
    }

    /// Extracts the ID from the response.
    ///
    /// Returns the request ID that this response corresponds to. If the ID is not
    /// present or cannot be parsed as a u64, returns 0 as a fallback.
    ///
    /// # Returns
    ///
    /// The response ID as a `u64`, or 0 if not found or invalid
    ///
    /// # Examples
    ///
    /// ```
    /// use ink_rpc::RpcResponse;
    /// let response = RpcResponse::new(42);
    /// assert_eq!(response.id(), 42);
    /// ```
    pub fn id(&self) -> u64 {
        self.value
            .get("id")
            .and_then(JsonValue::as_u64)
            .unwrap_or(0)
    }

    /// Returns a reference to the result field of the response.
    ///
    /// Gets the "result" field from the JSON-RPC response. If no result field
    /// is present, returns a reference to `JsonValue::Null`.
    ///
    /// # Returns
    ///
    /// A reference to the result `JsonValue`, or `&JsonValue::Null` if not present
    ///
    /// # Examples
    ///
    /// ```
    /// use ink_rpc::RpcResponse;
    /// use serde_json::json;
    ///
    /// let mut response = RpcResponse::new(1);
    /// response.set_result(json!("success"));
    /// assert_eq!(response.result(), &json!("success"));
    /// ```
    pub fn result(&self) -> &JsonValue {
        self.value.get("result").unwrap_or(&JsonValue::Null)
    }

    /// Sets the result field of the response.
    ///
    /// Updates the "result" field in the JSON-RPC response with the provided value.
    /// This is used to populate the response with the actual result data.
    ///
    /// # Arguments
    ///
    /// * `result` - The result value to set in the response
    ///
    /// # Examples
    ///
    /// ```
    /// use ink_rpc::RpcResponse;
    /// use serde_json::json;
    ///
    /// let mut response = RpcResponse::new(1);
    /// response.set_result(json!({"status": "ok", "data": [1, 2, 3]}));
    /// ```
    pub fn set_result(&mut self, result: JsonValue) {
        self.value["result"] = result;
    }

    /// Returns a reference to the error field of the response.
    ///
    /// Gets the "error" field from the JSON-RPC response. If no error field
    /// is present, returns a reference to `JsonValue::Null`.
    ///
    /// # Returns
    ///
    /// A reference to the error `JsonValue`, or `&JsonValue::Null` if not present
    ///
    /// # Examples
    ///
    /// ```
    /// use ink_rpc::RpcResponse;
    /// use serde_json::json;
    ///
    /// let mut response = RpcResponse::new(1);
    /// response.set_error(json!({"code": -1, "message": "Invalid request"}));
    /// assert_eq!(response.error(), &json!({"code": -1, "message": "Invalid request"}));
    /// ```
    pub fn error(&self) -> &JsonValue {
        self.value.get("error").unwrap_or(&JsonValue::Null)
    }

    /// Sets the error field of the response.
    ///
    /// Updates the "error" field in the JSON-RPC response with the provided value.
    /// This is used to populate the response with error information when a request fails.
    ///
    /// # Arguments
    ///
    /// * `error` - The error value to set in the response
    ///
    /// # Examples
    ///
    /// ```
    /// use ink_rpc::RpcResponse;
    /// use serde_json::json;
    ///
    /// let mut response = RpcResponse::new(1);
    /// response.set_error(json!({"code": -32600, "message": "Invalid Request"}));
    /// ```
    pub fn set_error(&mut self, error: JsonValue) {
        self.value["error"] = error;
    }
}

impl fmt::Display for RpcResponse {
    /// Formats the RPC response as a pretty-printed JSON string.
    ///
    /// This implementation of the `Display` trait converts the internal JSON value
    /// to a human-readable, indented JSON string. If serialization fails, it falls
    /// back to displaying "Null".
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to write the output to
    ///
    /// # Returns
    ///
    /// A `fmt::Result` indicating success or failure of the formatting operation
    ///
    /// # Examples
    ///
    /// ```
    /// use ink_rpc::RpcResponse;
    /// use serde_json::json;
    ///
    /// let mut response = RpcResponse::new(1);
    /// response.set_result(json!({"status": "ok"}));
    /// println!("{}", response); // Prints pretty-formatted JSON
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json_string = serde_json::to_string_pretty(&self.value).unwrap_or("Null".to_string());
        write!(f, "{}", &json_string)
    }
}

// ===========================================================================
// TESTS: RpcResponse
// ===

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_new_response() {
        let response = RpcResponse::new(123);

        assert_eq!(response.id(), 123);
        assert_eq!(response.as_json()["jsonrpc"], "2.0");
        assert_eq!(response.as_json()["id"], 123);
        assert!(!response.as_json().get("result").is_some());
    }

    #[test]
    fn test_new_response_zero_id() {
        let response = RpcResponse::new(0);

        assert_eq!(response.id(), 0);
        assert_eq!(response.as_json()["jsonrpc"], "2.0");
        assert_eq!(response.as_json()["id"], 0);
    }

    #[test]
    fn test_new_response_large_id() {
        let large_id = u64::MAX;
        let response = RpcResponse::new(large_id);

        assert_eq!(response.id(), large_id);
        assert_eq!(response.as_json()["id"], large_id);
    }

    #[test]
    fn test_from_json() {
        let json_value = json!({
            "jsonrpc": "2.0",
            "id": 456,
            "result": "success"
        });

        let response = RpcResponse::from_json(json_value.clone());

        assert_eq!(response.id(), 456);
        assert_eq!(response.result(), &json!("success"));
        assert_eq!(response.as_json(), &json_value);
    }

    #[test]
    fn test_from_json_with_error() {
        let json_value = json!({
            "jsonrpc": "2.0",
            "id": 789,
            "error": {
                "code": -1,
                "message": "Something went wrong"
            }
        });

        let response = RpcResponse::from_json(json_value.clone());

        assert_eq!(response.id(), 789);
        assert_eq!(response.as_json(), &json_value);
        // Result should be null when there's an error
        assert_eq!(response.result(), &JsonValue::Null);
    }

    #[test]
    fn test_as_json() {
        let mut response = RpcResponse::new(42);
        response.set_result(json!({"data": [1, 2, 3]}));

        let json_ref = response.as_json();
        assert_eq!(json_ref["jsonrpc"], "2.0");
        assert_eq!(json_ref["id"], 42);
        assert_eq!(json_ref["result"], json!({"data": [1, 2, 3]}));
    }

    #[test]
    fn test_id_extraction() {
        let response = RpcResponse::new(999);
        assert_eq!(response.id(), 999);
    }

    #[test]
    fn test_id_missing_returns_zero() {
        let json_without_id = json!({
            "jsonrpc": "2.0",
            "result": "test"
        });

        let response = RpcResponse::from_json(json_without_id);
        assert_eq!(response.id(), 0);
    }

    #[test]
    fn test_id_invalid_type_returns_zero() {
        let json_with_string_id = json!({
            "jsonrpc": "2.0",
            "id": "not_a_number",
            "result": "test"
        });

        let response = RpcResponse::from_json(json_with_string_id);
        assert_eq!(response.id(), 0);
    }

    #[test]
    fn test_result_getter() {
        let mut response = RpcResponse::new(1);

        // Initially result should be null
        assert_eq!(response.result(), &JsonValue::Null);

        // Set a string result
        response.set_result(json!("success"));
        assert_eq!(response.result(), &json!("success"));

        // Set an object result
        let complex_result = json!({
            "status": "ok",
            "data": {
                "items": [1, 2, 3],
                "count": 3
            }
        });
        response.set_result(complex_result.clone());
        assert_eq!(response.result(), &complex_result);
    }

    #[test]
    fn test_set_result_string() {
        let mut response = RpcResponse::new(1);
        response.set_result(json!("hello world"));

        assert_eq!(response.result(), &json!("hello world"));
        assert_eq!(response.as_json()["result"], "hello world");
    }

    #[test]
    fn test_set_result_number() {
        let mut response = RpcResponse::new(1);
        response.set_result(json!(42));

        assert_eq!(response.result(), &json!(42));
        assert_eq!(response.as_json()["result"], 42);
    }

    #[test]
    fn test_set_result_boolean() {
        let mut response = RpcResponse::new(1);
        response.set_result(json!(true));

        assert_eq!(response.result(), &json!(true));
        assert_eq!(response.as_json()["result"], true);
    }

    #[test]
    fn test_set_result_array() {
        let mut response = RpcResponse::new(1);
        let array_result = json!([1, "two", 3.0, true]);
        response.set_result(array_result.clone());

        assert_eq!(response.result(), &array_result);
        assert_eq!(response.as_json()["result"], array_result);
    }

    #[test]
    fn test_set_result_object() {
        let mut response = RpcResponse::new(1);
        let object_result = json!({
            "name": "John Doe",
            "age": 30,
            "active": true,
            "scores": [95, 87, 92]
        });
        response.set_result(object_result.clone());

        assert_eq!(response.result(), &object_result);
        assert_eq!(response.as_json()["result"], object_result);
    }

    #[test]
    fn test_set_result_null() {
        let mut response = RpcResponse::new(1);
        response.set_result(JsonValue::Null);

        assert_eq!(response.result(), &JsonValue::Null);
        assert_eq!(response.as_json()["result"], JsonValue::Null);
    }

    #[test]
    fn test_set_result_overwrites_previous() {
        let mut response = RpcResponse::new(1);

        response.set_result(json!("first"));
        assert_eq!(response.result(), &json!("first"));

        response.set_result(json!("second"));
        assert_eq!(response.result(), &json!("second"));

        response.set_result(json!({"final": "value"}));
        assert_eq!(response.result(), &json!({"final": "value"}));
    }

    #[test]
    fn test_error_getter() {
        let mut response = RpcResponse::new(1);

        // Initially error should be null
        assert_eq!(response.error(), &JsonValue::Null);

        // Set a simple error
        response.set_error(json!({"code": -1, "message": "Test error"}));
        assert_eq!(
            response.error(),
            &json!({"code": -1, "message": "Test error"})
        );

        // Set a complex error with additional data
        let complex_error = json!({
            "code": -32600,
            "message": "Invalid Request",
            "data": {
                "details": "Missing required parameter",
                "line": 42
            }
        });
        response.set_error(complex_error.clone());
        assert_eq!(response.error(), &complex_error);
    }

    #[test]
    fn test_set_error_standard_codes() {
        let mut response = RpcResponse::new(1);

        // Test parse error
        response.set_error(json!({"code": -32700, "message": "Parse error"}));
        assert_eq!(
            response.error(),
            &json!({"code": -32700, "message": "Parse error"})
        );
        assert_eq!(response.as_json()["error"]["code"], -32700);

        // Test invalid request
        response.set_error(json!({"code": -32600, "message": "Invalid Request"}));
        assert_eq!(
            response.error(),
            &json!({"code": -32600, "message": "Invalid Request"})
        );
        assert_eq!(response.as_json()["error"]["code"], -32600);

        // Test method not found
        response.set_error(json!({"code": -32601, "message": "Method not found"}));
        assert_eq!(
            response.error(),
            &json!({"code": -32601, "message": "Method not found"})
        );
        assert_eq!(response.as_json()["error"]["code"], -32601);
    }

    #[test]
    fn test_set_error_with_data() {
        let mut response = RpcResponse::new(1);
        let error_with_data = json!({
            "code": -32602,
            "message": "Invalid params",
            "data": {
                "expected": "string",
                "received": "number",
                "parameter": "username"
            }
        });
        response.set_error(error_with_data.clone());

        assert_eq!(response.error(), &error_with_data);
        assert_eq!(response.as_json()["error"], error_with_data);
    }

    #[test]
    fn test_set_error_null() {
        let mut response = RpcResponse::new(1);
        response.set_error(JsonValue::Null);

        assert_eq!(response.error(), &JsonValue::Null);
        assert_eq!(response.as_json()["error"], JsonValue::Null);
    }

    #[test]
    fn test_set_error_overwrites_previous() {
        let mut response = RpcResponse::new(1);

        response.set_error(json!({"code": -1, "message": "first error"}));
        assert_eq!(
            response.error(),
            &json!({"code": -1, "message": "first error"})
        );

        response.set_error(json!({"code": -2, "message": "second error"}));
        assert_eq!(
            response.error(),
            &json!({"code": -2, "message": "second error"})
        );

        response.set_error(json!({"code": -32700, "message": "Parse error"}));
        assert_eq!(
            response.error(),
            &json!({"code": -32700, "message": "Parse error"})
        );
    }

    #[test]
    fn test_error_and_result_independence() {
        let mut response = RpcResponse::new(1);

        // Set result first
        response.set_result(json!("success"));
        assert_eq!(response.result(), &json!("success"));
        assert_eq!(response.error(), &JsonValue::Null);

        // Set error - should not affect result
        response.set_error(json!({"code": -1, "message": "error"}));
        assert_eq!(response.result(), &json!("success"));
        assert_eq!(response.error(), &json!({"code": -1, "message": "error"}));

        // Update result - should not affect error
        response.set_result(json!("updated"));
        assert_eq!(response.result(), &json!("updated"));
        assert_eq!(response.error(), &json!({"code": -1, "message": "error"}));
    }
}
