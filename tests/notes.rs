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
            "stylo_id": "123e4567-e89b-12d3-a456-426614174000",
            "content": "This is a test note"
        }))
        .await;

    assert_eq!(response.status_code(), 201);

    let location = response.header("Location");
    assert!(location.to_str().unwrap().starts_with("/note/"));
}

#[tokio::test]
async fn test_create_project_success() {
    let mut container = Container::default();
    let client = initialize_test_server(&mut container).await;

    let response = client
        .post("/project/create")
        .json(&json!({
            "universe_id": "123e4567-e89b-12d3-a456-426614174000",
            "project_name": "New Project"
        }))
        .await;

    assert_eq!(response.status_code(), 201);

    let location = response.header("Location");
    assert!(location
        .to_str()
        .unwrap()
        .starts_with("/project/new-project"));

    // Verify the project was created
    let project_book = container.project_book().unwrap();
    let project = project_book
        .get_by_slug("new-project")
        .await
        .unwrap()
        .expect("there should be a project");
    assert_eq!(project.project_name, "New Project");
}

#[tokio::test]
async fn test_scratch_note_success() {
    let mut container = Container::default();
    let project_book = container.project_book().unwrap();
    let note_book = container.note_book().unwrap();

    // Create a project
    let project_command = kaku::models::CreateProjectCommand {
        universe_id: Uuid::new_v4(),
        project_name: "Test Project".to_string(),
    };
    let project = project_book.create(project_command).await.unwrap();

    // Create a note
    let note_command = kaku::models::CreateNoteCommand {
        imported_at: chrono::Utc::now(),
        stylo_id: Uuid::new_v4(),
        project_slug: project.slug.clone(),
        content: "This is a test note".to_string(),
    };
    let note = note_book
        .add(note_command, project.project_id)
        .await
        .unwrap();

    let client = initialize_test_server(&mut container).await;

    // Scratch the note
    let response = client.delete(&format!("/notes/{}", note.note_id)).await;

    assert_eq!(response.status_code(), 204);

    // Verify the note was scratched
    assert!(note_book.get(note.note_id).await.unwrap().is_none());
}

#[tokio::test]
async fn test_create_thought_success() {
    let mut container = Container::default();
    let project_book = container.project_book().unwrap();

    // Create a project first
    let project_command = kaku::models::CreateProjectCommand {
        universe_id: Uuid::new_v4(),
        project_name: "Test Project".to_string(),
    };
    project_book.create(project_command).await.unwrap();

    let client = initialize_test_server(&mut container).await;

    let stylo_id = Uuid::new_v4();
    let response = client
        .post("/project/test-project/thought")
        .json(&json!({
            "imported_at": "2023-10-01T12:00:00Z",
            "stylo_id": stylo_id,
            "content": "This is a test thought"
        }))
        .await;

    assert_eq!(response.status_code(), 201);
    let location = response.header("Location");
    assert!(location.to_str().unwrap().starts_with("/thought/"));
}
