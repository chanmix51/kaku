// Tests for the notes endpoint
use axum_test::TestServer;
use kaku::{actor::ApiApp, Container};
use serde_json::json;
use uuid::Uuid;

async fn initialize_test_server(container: &mut Container) -> TestServer {
    let service = container.thought_service().unwrap();
    let app = ApiApp::new(service).router();
    TestServer::new(app).unwrap()
}

#[tokio::test]
async fn test_create_note_success() {
    let mut container = Container::default();
    let project_book = container.project_book().unwrap();
    let project_command = kaku::models::CreateProjectCommand {
        universe_id: Uuid::new_v4(),
        project_name: "Whatever".to_string(),
    };
    project_book.create(project_command).await.unwrap();
    let client = initialize_test_server(&mut container).await;

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
