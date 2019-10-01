use super::prelude::*;
use dawn_model::id::WebhookId;

#[derive(Serialize)]
pub struct DeleteWebhook<'a> {
    token: Option<String>,
    #[serde(skip)]
    fut: Option<Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>>>,
    #[serde(skip)]
    http: &'a Client,
    id: WebhookId,
}

impl<'a> DeleteWebhook<'a> {
    pub(crate) fn new(http: &'a Client, id: impl Into<WebhookId>) -> Self {
        Self {
            fut: None,
            http,
            id: id.into(),
            token: None,
        }
    }

    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token.replace(token.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.verify(Request::from(
            Route::DeleteWebhook {
                webhook_id: self.id.0,
                token: self.token.as_ref().map(ToOwned::to_owned),
            },
        ))));

        Ok(())
    }
}

poll_req!(DeleteWebhook<'_>, ());