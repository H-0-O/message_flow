use message_flow::{DefaultResponseErrorSchema, Message};

#[test]
fn test_default_error_schema_creation() {
    let error = DefaultResponseErrorSchema {
        code: 404,
        message: "Not Found".to_string(),
    };

    assert_eq!(error.code, 404);
    assert_eq!(error.message, "Not Found");
}

#[test]
fn test_default_error_schema_serialization() {
    let error = DefaultResponseErrorSchema {
        code: 500,
        message: "Internal Server Error".to_string(),
    };

    let json = error.to_json();
    assert!(json.contains("500"));
    assert!(json.contains("Internal Server Error"));

    // Verify it's valid JSON
    let deserialized: DefaultResponseErrorSchema = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.code, 500);
    assert_eq!(deserialized.message, "Internal Server Error");
}

#[test]
fn test_error_schema_implements_message() {
    let error = DefaultResponseErrorSchema {
        code: 400,
        message: "Bad Request".to_string(),
    };

    // Should compile and work with Message trait
    let json = error.to_json();
    assert!(!json.is_empty());
}

#[test]
fn test_various_error_codes() {
    let codes = vec![200, 400, 404, 500, 503];

    for code in codes {
        let error = DefaultResponseErrorSchema {
            code,
            message: format!("Error {}", code),
        };

        let json = error.to_json();
        assert!(json.contains(&code.to_string()));
    }
}
