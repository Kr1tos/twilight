use crate::request::prelude::*;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::id::ChannelId;

/// The request can not be created as configured.
#[derive(Debug)]
pub struct CreateStageInstanceError {
    kind: CreateStageInstanceErrorType,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl CreateStageInstanceError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub fn kind(&self) -> &CreateStageInstanceErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        self.source
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(
        self,
    ) -> (
        CreateStageInstanceErrorType,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, self.source)
    }
}

impl Display for CreateStageInstanceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            CreateStageInstanceErrorType::InvalidTopic { .. } => f.write_str("invalid topic"),
        }
    }
}

impl Error for CreateStageInstanceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

#[derive(Debug)]
pub enum CreateStageInstanceErrorType {
    /// Topic is not between 1 and 120 characters in length.
    InvalidTopic {
        /// Invalid topic.
        topic: String,
    },
}

/// Create a new stage instance associated with a stage channel.
///
/// Requires the user to be a moderator of the stage channel.
pub struct CreateStageInstance<'a> {
    channel_id: ChannelId,
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
    topic: String,
}

impl<'a> CreateStageInstance<'a> {
    pub(crate) fn new(
        http: &'a Client,
        channel_id: ChannelId,
        topic: impl Into<String>,
    ) -> Result<Self, CreateStageInstanceError> {
        Self::_new(http, channel_id, topic.into())
    }

    fn _new(
        http: &'a Client,
        channel_id: ChannelId,
        topic: String,
    ) -> Result<Self, CreateStageInstanceError> {
        if !validate::stage_topic(&topic) {
            return Err(CreateStageInstanceError {
                kind: CreateStageInstanceErrorType::InvalidTopic { topic },
                source: None,
            });
        }

        Ok(Self {
            channel_id,
            fut: None,
            http,
            topic,
        })
    }

    fn start(&mut self) -> Result<()> {
        let request = Request::from_route(Route::CreateStageInstance {
            channel_id: self.channel_id.0,
            topic: self.topic.clone(),
        });

        self.fut.replace(Box::pin(self.http.verify(request)));

        Ok(())
    }
}

poll_req!(CreateStageInstance<'_>, ());