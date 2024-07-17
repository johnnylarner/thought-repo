use crate::domain::blog::models::blog::Blog;
use crate::domain::blog::ports::BlogNotifier;

/// An unimplemented example of an adapter to [BlogNotifier].
#[derive(Debug, Clone)]
pub struct EmailClient;

impl EmailClient {
    pub fn new() -> Self {
        Self
    }
}

impl BlogNotifier for EmailClient {
    async fn blog_failed(&self, _: &Blog) {}
    async fn blog_created(&self, _: &Blog) {}
}
