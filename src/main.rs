use clap::Parser;
use log::warn;
use log::{debug, error, info, log_enabled, Level};
use std::sync::Arc;
use tokio::signal;
use tokio::task::JoinHandle;

use kaku::actor::ApiApp;
use kaku::adapter::InMemoryNoteBook;
use kaku::adapter::InMemoryProjectBook;
use kaku::service::ThoughtService;
use kaku::Result;

/// Application configuration
#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub struct Config {
    /// API server host
    #[arg(long, default_value = "127.0.0.1")]
    pub host: String,

    /// API server port
    #[arg(long, default_value = "8080")]
    pub port: u16,
}

/// Application
/// It configures and launches the different actors of the application.
pub struct Application {
    config: Config,
}

impl Application {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Run the application
    /// It launches the API server and waits for a signal to stop the application.
    pub async fn run(self) -> Result<()> {
        let note_book = Arc::new(InMemoryNoteBook::default());
        let project_book = Arc::new(InMemoryProjectBook::default());
        let thought_service = Arc::new(ThoughtService::new(note_book, project_book));
        let api_app = ApiApp::new(thought_service.clone());

        let joinhandle: JoinHandle<Result<()>> = tokio::spawn(async move {
            let addr = format!("{}:{}", self.config.host, self.config.port);
            let router = api_app.router();
            let listener = tokio::net::TcpListener::bind(&addr).await?;
            debug!("Listening on: {addr}");
            axum::serve(listener, router).await?;

            Ok(())
        });

        tokio::select! {
            r = joinhandle => {r?},
            _ = signal::ctrl_c() => {
                warn!("Received Ctrl+C, shutting down...");
                Ok(())
            },
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    info!("Starting Kaku.");
    let config = Config::parse();
    let app = Application::new(config);

    match app.run().await {
        Ok(_) => {
            info!("Kaku stopped.");
            Ok(())
        }
        Err(e) => {
            error!("Kaku stopped with error: {e}");
            Err(e)
        }
    }
}
