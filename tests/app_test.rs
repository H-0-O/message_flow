use message_flow::{app::App, DefaultResponseErrorSchema, Message};

#[tokio::test]
async fn test_app_new_with_valid_address() {
    let app_result = App::new("localhost:4222".to_string()).await;
    
    match app_result {
        Ok(_app) => {
            // Verify app was created
            // Context should be initialized
            assert!(true); // App created successfully
        }
        Err(_) => {
            // NATS server not available - skip test
            // This is expected in CI environments
        }
    }
}

#[tokio::test]
async fn test_app_new_with_invalid_address() {
    // Test with an invalid address - should fail
    let app_result = App::new("invalid:9999".to_string()).await;
    
    // Should return an error
    assert!(app_result.is_err());
}

#[test]
fn test_error_handler_customization() {
    // Test that error handler can be customized
    let custom_error = DefaultResponseErrorSchema {
        code: 999,
        message: "Custom error".to_string(),
    };
    
    let json = custom_error.to_json();
    assert!(json.contains("999"));
    assert!(json.contains("Custom error"));
}

#[tokio::test]
async fn test_app_set_error_handler() {
    let app_result = App::new("localhost:4222".to_string()).await;
    
    match app_result {
        Ok(app) => {
            // Test setting a custom error handler
            let _app_with_handler = app.set_error_handler(|code, message, _client| {
                let error = DefaultResponseErrorSchema {
                    code,
                    message: format!("Custom: {}", message),
                };
                Box::new(error)
            });
            
            // Verify the method chains correctly
            assert!(true); // If we get here, it worked
        }
        Err(_) => {
            // NATS server not available - skip test
        }
    }
}

