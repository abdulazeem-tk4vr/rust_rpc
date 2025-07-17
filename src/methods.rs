use crate::api::{RpcRequest, RpcResponse};

pub fn lib_dummy(payload: RpcRequest) -> RpcResponse {
    let result = serde_json::json!(payload.method);
    RpcResponse {
        jsonrpc: "2.0".to_string(),
        result: Some(result),
        error: None,
        id: payload.id,
    }
}
