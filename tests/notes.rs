use std::sync::Arc;

use axum_test::TestServer;
use kaku::{actor::ApiApp, adapter::InMemoryNoteBook};
use serde_json::json;

async fn initialize_test_server() -> TestServer {
    let service = kaku::service::ThoughtService::new(Arc::new(InMemoryNoteBook::default()));
    let app = ApiApp::new(Arc::new(service)).router();
    TestServer::new(app).unwrap()
}

#[tokio::test]
async fn test_create_note_success() {
    let client = initialize_test_server().await;

    let response = client
        .post("/project/whatever/note")
        .json(&json!({
            "imported_at": "2023-10-01T12:00:00Z",
            "scribe_id": "123e4567-e89b-12d3-a456-426614174000",
            "content": "This is a test note"
        }))
        .await;

    assert_eq!(response.status_code(), 201);
}
