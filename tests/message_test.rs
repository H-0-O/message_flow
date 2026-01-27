use message_flow::Message;
use message_flow_drive::MsgDef;
use serde::{Deserialize, Serialize};

#[derive(MsgDef, Debug, Serialize, Deserialize)]
struct User {
    name: String,
    age: u32,
}

#[derive(MsgDef, Debug, Serialize, Deserialize)]
struct ComplexMessage {
    id: u64,
    tags: Vec<String>,
    metadata: Option<String>,
}

#[test]
fn test_basic_message_serialization() {
    let user = User {
        name: "Alice".to_string(),
        age: 30,
    };
    
    let json = user.to_json();
    assert!(json.contains("Alice"));
    assert!(json.contains("30"));
    
    // Verify it's valid JSON
    let deserialized: User = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.name, "Alice");
    assert_eq!(deserialized.age, 30);
}

#[test]
fn test_complex_message_serialization() {
    let msg = ComplexMessage {
        id: 12345,
        tags: vec!["rust".to_string(), "testing".to_string()],
        metadata: Some("test metadata".to_string()),
    };
    
    let json = msg.to_json();
    assert!(json.contains("12345"));
    assert!(json.contains("rust"));
    assert!(json.contains("testing"));
    assert!(json.contains("test metadata"));
    
    // Test with None metadata
    let msg2 = ComplexMessage {
        id: 67890,
        tags: vec![],
        metadata: None,
    };
    
    let json2 = msg2.to_json();
    assert!(json2.contains("67890"));
    assert!(json2.contains("null")); // None serializes as null
}

#[test]
fn test_message_with_special_characters() {
    let user = User {
        name: "O'Brien".to_string(),
        age: 25,
    };
    
    let json = user.to_json();
    let deserialized: User = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.name, "O'Brien");
}

#[test]
fn test_message_round_trip() {
    let original = User {
        name: "Bob".to_string(),
        age: 42,
    };
    
    let json = original.to_json();
    let deserialized: User = serde_json::from_str(&json).unwrap();
    
    assert_eq!(original.name, deserialized.name);
    assert_eq!(original.age, deserialized.age);
}

