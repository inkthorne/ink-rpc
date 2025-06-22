use ink_rpc::{RpcRequest, RpcResponse};
use serde_json::json;

fn main() {
    println!("=== JSON-RPC Request and Response Example ===\n");

    // Example 1: Simple request with null parameters
    println!("Example 1: Simple request with null parameters");
    println!("----------------------------------------------");
    
    let mut simple_request = RpcRequest::new();
    simple_request.set_method("get_server_info".to_string());
    
    println!("Request:");
    println!("{}\n", simple_request);
    
    // Create corresponding response
    let mut simple_response = RpcResponse::new(simple_request.id());
    simple_response.set_result(json!({
        "server": "ink-rpc-server",
        "version": "1.0.0",
        "uptime": 3600
    }));
    
    println!("Response:");
    println!("{}\n", simple_response);

    // Example 2: Request with complex parameters
    println!("Example 2: Request with complex parameters");
    println!("------------------------------------------");
    
    let mut transfer_request = RpcRequest::new();
    transfer_request
        .set_method("transfer_funds".to_string())
        .set_params(json!({
            "from_account": "acc_123456",
            "to_account": "acc_789012",
            "amount": 250.75,
            "currency": "USD",
            "memo": "Payment for invoice #INV-2025-001"
        }));
    
    println!("Request:");
    println!("{}\n", transfer_request);
    
    // Create successful response
    let mut transfer_response = RpcResponse::new(transfer_request.id());
    transfer_response.set_result(json!({
        "transaction_id": "txn_abc123def456",
        "status": "completed",
        "timestamp": "2025-06-22T10:30:00Z",
        "final_balance": 1749.25
    }));
    
    println!("Response:");
    println!("{}\n", transfer_response);

    // Example 3: Request that results in an error
    println!("Example 3: Request that results in an error");
    println!("--------------------------------------------");
    
    let mut invalid_request = RpcRequest::new();
    invalid_request
        .set_method("withdraw_funds".to_string())
        .set_params(json!({
            "account": "acc_123456",
            "amount": 5000.0  // Amount exceeds balance
        }));
    
    println!("Request:");
    println!("{}\n", invalid_request);
    
    // Create error response
    let mut error_response = RpcResponse::new(invalid_request.id());
    error_response.set_error(json!({
        "code": -32001,
        "message": "Insufficient funds",
        "data": {
            "requested_amount": 5000.0,
            "available_balance": 1749.25,
            "account": "acc_123456"
        }
    }));
    
    println!("Error Response:");
    println!("{}\n", error_response);

    // Example 4: Batch request simulation
    println!("Example 4: Batch request simulation");
    println!("-----------------------------------");
    
    let requests = vec![
        {
            let mut req = RpcRequest::new();
            req.set_method("get_balance".to_string())
               .set_params(json!({"account": "acc_123456"}));
            req
        },
        {
            let mut req = RpcRequest::new();
            req.set_method("get_transaction_history".to_string())
               .set_params(json!({
                   "account": "acc_123456",
                   "limit": 5
               }));
            req
        },
        {
            let mut req = RpcRequest::new();
            req.set_method("get_account_info".to_string())
               .set_params(json!({"account": "acc_123456"}));
            req
        }
    ];
    
    // Print all requests
    for (i, request) in requests.iter().enumerate() {
        println!("Batch Request {}:", i + 1);
        println!("{}", request);
    }
    
    // Create corresponding responses
    let responses = vec![
        {
            let mut resp = RpcResponse::new(requests[0].id());
            resp.set_result(json!({"balance": 1749.25, "currency": "USD"}));
            resp
        },
        {
            let mut resp = RpcResponse::new(requests[1].id());
            resp.set_result(json!({
                "transactions": [
                    {
                        "id": "txn_abc123def456",
                        "type": "credit",
                        "amount": 250.75,
                        "timestamp": "2025-06-22T10:30:00Z"
                    },
                    {
                        "id": "txn_def456ghi789",
                        "type": "debit",
                        "amount": 50.0,
                        "timestamp": "2025-06-21T14:15:00Z"
                    }
                ]
            }));
            resp
        },
        {
            let mut resp = RpcResponse::new(requests[2].id());
            resp.set_result(json!({
                "account_id": "acc_123456",
                "account_type": "checking",
                "status": "active",
                "created_date": "2023-01-15T08:00:00Z"
            }));
            resp
        }
    ];
    
    // Print all responses
    for (i, response) in responses.iter().enumerate() {
        println!("Batch Response {}:", i + 1);
        println!("{}", response);
    }

    println!("=== Example Complete ===");
}
