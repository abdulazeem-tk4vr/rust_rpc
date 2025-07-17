use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct RpcRequest {
    /// JSON-RPC protocol version.
    pub jsonrpc: String,

    /// The name of the method being invoked.
    pub method: String,

    /// Parameters for the method call, if any.
    pub params: Option<Value>,

    /// Unique identifier for the request.
    pub id: u32,
}

/// Represents a JSON-RPC response.
#[derive(Debug, Serialize, Deserialize)]
pub struct RpcResponse {
    /// JSON-RPC protocol version.
    pub jsonrpc: String,

    /// The result of the method call, if successful.
    pub result: Option<Value>,

    /// The error object, if the method call failed.
    pub error: Option<RpcError>,

    /// Unique identifier for the response.
    pub id: u32,
}

/// Represents an error response for JSON-RPC.
#[derive(Debug, Serialize, Deserialize)]
pub struct RpcError {
    /// Error code.
    code: i32,

    /// Error message.
    message: String,
}

pub fn generate_error_response(error_code: i32, error_msg: String, id: u32) -> RpcResponse {
    RpcResponse {
        jsonrpc: "2.0".to_string(),
        result: None,
        error: Some(RpcError {
            code: error_code,
            message: error_msg,
        }),
        id,
    }
}
