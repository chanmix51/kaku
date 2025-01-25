use std::cell::OnceCell;
use std::sync::Arc;

use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::Result;

/// Dependencies injection container
/// All dependencies are stored in this container each in a OnceCell.
#[derive(Default)]
pub struct Container {
    note_book: OnceCell<Arc<dyn crate::adapter::NoteBook>>,
    project_book: OnceCell<Arc<dyn crate::adapter::ProjectBook>>,
    thought_service: OnceCell<Arc<crate::service::ThoughtService>>,
    event_dispatcher: OnceCell<Arc<synapps::EventDispatcher<crate::models::ModelEvent>>>,
    event_publisher: OnceCell<(
        UnboundedSender<crate::models::ModelEvent>,
        UnboundedReceiver<crate::models::ModelEvent>,
    )>,
}

impl Container {
    /// Get or iniitalize the channels for the event
    fn event_publisher(
        &mut self,
    ) -> Result<&(
        UnboundedSender<crate::models::ModelEvent>,
        UnboundedReceiver<crate::models::ModelEvent>,
    )> {
        Ok(self
            .event_publisher
            .get_or_init(tokio::sync::mpsc::unbounded_channel))
    }

    /// get the event publisher
    /// it returns only the publisher but stores the couple (sender, receiver) in the container
    pub fn event_publisher_sender(&mut self) -> Result<UnboundedSender<crate::models::ModelEvent>> {
        Ok(self.event_publisher()?.0.clone())
    }

    /// get the event receiver for the event dispatcher
    pub fn event_publisher_receiver(
        &mut self,
    ) -> Result<UnboundedReceiver<crate::models::ModelEvent>> {
        let _ = self.event_publisher()?;
        let receiver = self.event_publisher.take().unwrap().1;

        Ok(receiver)
    }

    /// Get the note book
    pub fn note_book(&mut self) -> Result<Arc<dyn crate::adapter::NoteBook>> {
        Ok(self
            .note_book
            .get_or_init(|| Arc::new(crate::adapter::InMemoryNoteBook::default()))
            .clone())
    }

    /// Get the project book
    pub fn project_book(&mut self) -> Result<Arc<dyn crate::adapter::ProjectBook>> {
        Ok(self
            .project_book
            .get_or_init(|| Arc::new(crate::adapter::InMemoryProjectBook::default()))
            .clone())
    }

    /// Get the thought service
    pub fn thought_service(&mut self) -> Result<Arc<crate::service::ThoughtService>> {
        let note_book = self.note_book()?;
        let project_book = self.project_book()?;
        let sender = self.event_publisher_sender()?;

        Ok(self
            .thought_service
            .get_or_init(|| {
                Arc::new(crate::service::ThoughtService::new(
                    note_book,
                    project_book,
                    sender,
                ))
            })
            .clone())
    }
}
