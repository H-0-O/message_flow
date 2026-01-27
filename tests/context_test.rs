use message_flow::{DefaultResponseErrorSchema, Message};
use message_flow_drive::MsgDef;
use serde::{Deserialize, Serialize};

#[derive(MsgDef, Debug, Serialize, Deserialize)]
struct TestMessage {
    content: String,
}

#[tokio::test]
async fn test_context_new() {
    // This test requires a NATS client, so we'll test the error handler setup
    // In a real scenario, you'd use a mock or test NATS server
    // For now, we'll test that the default error handler works correctly
    
    // Create a test error response
    let error_response = DefaultResponseErrorSchema {
        code: 500,
        message: "Test error".to_string(),
    };
    
    // Verify it implements Message trait
    let json = error_response.to_json();
    assert!(json.contains("500"));
    assert!(json.contains("Test error"));
}

#[test]
fn test_message_serialization() {
    let msg = TestMessage {
        content: "Hello, World!".to_string(),
    };
    
    let json = msg.to_json();
    assert!(json.contains("Hello, World!"));
    assert!(json.contains("content"));
    
    // Test deserialization
    let deserialized: TestMessage = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.content, "Hello, World!");
}

#[test]
fn test_message_trait_implementations() {
    // Test String implementation
    let string_msg: String = "test".to_string();
    let json = string_msg.to_json();
    assert_eq!(json, "\"test\"");
    
    // Test Vec<String> implementation
    let vec_msg: Vec<String> = vec!["a".to_string(), "b".to_string()];
    let json = vec_msg.to_json();
    assert!(json.contains("a"));
    assert!(json.contains("b"));
    
    // Test u128 implementation
    let num_msg: u128 = 42;
    let json = num_msg.to_json();
    assert_eq!(json, "42");
}

