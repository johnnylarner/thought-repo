/*!
   Module `service` provides the canonical implementation of the [BlogService] port. All
   blog-domain logic is defined here.
*/
use crate::domain::blog::models::blog::{Blog, CreateBlogError, CreateBlogRequest};
use crate::domain::blog::ports::{BlogNotifier, BlogPublisher, BlogRepository, BlogService};

/// Canonical implementation of the [BlogService] port, through which the blog domain API is
/// consumed.
#[derive(Debug, Clone)]
pub struct Service<R, P, N>
where
    R: BlogRepository,
    P: BlogPublisher,
    N: BlogNotifier,
{
    repo: R,
    publisher: P,
    blog_notifier: N,
}

impl<R, P, N> Service<R, P, N>
where
    R: BlogRepository,
    P: BlogPublisher,
    N: BlogNotifier,
{
    pub fn new(repo: R, metrics: P, blog_notifier: N) -> Self {
        Self {
            repo,
            publisher: metrics,
            blog_notifier,
        }
    }
}

impl<R, P, N> BlogService for Service<R, P, N>
where
    R: BlogRepository,
    P: BlogPublisher,
    N: BlogNotifier,
{
    /// Create the [blog] specified in `req` and trigger notifications.
    ///
    /// # Errors
    ///
    /// - Propagates any [CreateblogError] returned by the [BlogRepository].
    async fn create_blog(&self, req: &CreateBlogRequest) -> Result<Blog, CreateBlogError> {
        let result = self.repo.create_blog(req).await;
        if result.is_err() {
            self.blog_notifier
                .blog_failed(result.as_ref().unwrap())
                .await;
        } else {
            self.publisher.publish_to_dev_to().await;
            self.blog_notifier
                .blog_created(result.as_ref().unwrap())
                .await;
        }

        result
    }
}
