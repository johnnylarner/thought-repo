/*
   Module `ports` specifies the API by which external modules interact with the blog domain.

   All traits are bounded by `Send + Sync + 'static`, since their implementations must be shareable
   between request-handling threads.

   Trait methods are explicitly asynchronous, including `Send` bounds on response types,
   since the application is expected to always run in a multithreaded environment.
*/

use std::future::Future;

use crate::domain::blog::models::blog::{Blog, CreateBlogError, CreateBlogRequest};

/// `BlogService` is the public API for the blog domain.
///
/// External modules must conform to this contract – the domain is not concerned with the
/// implementation details or underlying technology of any external code.
pub trait BlogService: Clone + Send + Sync + 'static {
    /// Asynchronously create a new [Blog].
    ///
    /// # Errors
    ///
    /// - [CreateBlogError::Duplicate] if an [Blog] with the same [BlogName] already exists.
    fn create_blog(
        &self,
        req: &CreateBlogRequest,
    ) -> impl Future<Output = Result<Blog, CreateBlogError>> + Send;
}

/// `BlogRepository` represents a store of blog data.
///
/// External modules must conform to this contract – the domain is not concerned with the
/// implementation details or underlying technology of any external code.
pub trait BlogRepository: Send + Sync + Clone + 'static {
    /// Asynchronously persist a new [Blog].
    ///
    /// # Errors
    ///
    /// - MUST return [CreateBlogError::Duplicate] if an [Blog] with the same [BlogName]
    ///   already exists.
    fn create_blog(
        &self,
        req: &CreateBlogRequest,
    ) -> impl Future<Output = Result<Blog, CreateBlogError>> + Send;
}

/// `BlogMetrics` describes an aggregator of blog-related metrics, such as a time-series
/// database.
pub trait BlogPublisher: Send + Sync + Clone + 'static {
    /// Publishes a [Blog] to `dev.to`
    fn publish_to_dev_to(&self) -> impl Future<Output = ()> + Send;
}

/// `BlogNotifier` triggers notifications to blogs.
///
/// For others, code coordinating notifications will be complex enough to warrant its own domain.
/// In this case, an `BlogNotifier` adapter will call that domain's `Service`.
pub trait BlogNotifier: Send + Sync + Clone + 'static {
    fn blog_failed(&self, blog: &Blog) -> impl Future<Output = ()> + Send;
    fn blog_created(&self, blog: &Blog) -> impl Future<Output = ()> + Send;
}
