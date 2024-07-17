/*
   Module `create_blog` specifies an HTTP handler for creating a new [Blog], and the
   associated data structures.
*/

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::domain::blog::models::blog::{Blog, CreateBlogError, CreateBlogRequest};
use crate::domain::blog::ports::BlogService;
use crate::inbound::http::responses::{ApiError, ApiSuccess};
use crate::inbound::http::AppState;

impl From<CreateBlogError> for ApiError {
    fn from(e: CreateBlogError) -> Self {
        match e {
            CreateBlogError::Unknown(cause) => {
                tracing::error!("{:?}\n{}", cause, cause.backtrace());
                Self::InternalServerError("Internal server error".to_string())
            }
        }
    }
}

impl From<ParseCreateBlogHttpRequestError> for ApiError {
    fn from(e: ParseCreateBlogHttpRequestError) -> Self {
        Self::UnprocessableEntity("unknown error".to_string())
    }
}

/// The body of an [Blog] creation request.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CreateBlogRequestBody {
    name: String,
}

/// The response body data field for successful [Blog] creation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CreateBlogResponseData {
    id: String,
}

impl From<&Blog> for CreateBlogResponseData {
    fn from(blog: &Blog) -> Self {
        Self {
            id: "placeholder".to_string(),
        }
    }
}

/// The body of an [Blog] creation request.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CreateBlogHttpRequestBody {
    name: String,
    email_address: String,
}

#[derive(Debug, Clone, Error)]
enum ParseCreateBlogHttpRequestError {}

impl CreateBlogHttpRequestBody {
    /// Converts the HTTP request body into a domain request.
    fn try_into_domain(self) -> Result<CreateBlogRequest, ParseCreateBlogHttpRequestError> {
        Ok(CreateBlogRequest)
    }
}

/// Create a new [Blog].
///
/// # Responses
///
/// - 201 Created: the [Blog] was successfully created.
/// - 422 Unprocessable entity: An [Blog] with the same name already exists.
pub async fn create_blog<BS: BlogService>(
    State(state): State<AppState<BS>>,
    Json(body): Json<CreateBlogHttpRequestBody>,
) -> Result<ApiSuccess<CreateBlogResponseData>, ApiError> {
    let domain_req = body.try_into_domain()?;
    state
        .blog_service
        .create_blog(&domain_req)
        .await
        .map_err(ApiError::from)
        .map(|ref blog| ApiSuccess::new(StatusCode::CREATED, blog.into()))
}

#[cfg(test)]
mod tests {
    use std::mem;
    use std::sync::Arc;

    use anyhow::anyhow;
    use uuid::Uuid;

    use crate::domain::blog::models::blog::CreateBlogError;
    use crate::domain::blog::models::blog::{Blog, CreateBlogRequest};
    use crate::domain::blog::ports::BlogService;

    use super::*;

    #[derive(Clone)]
    struct MockBlogService {
        create_blog_result: Arc<std::sync::Mutex<Result<Blog, CreateBlogError>>>,
    }

    impl BlogService for MockBlogService {
        async fn create_blog(&self, _: &CreateBlogRequest) -> Result<Blog, CreateBlogError> {
            let mut guard = self.create_blog_result.lock();
            let mut result = Err(CreateBlogError::Unknown(anyhow!("substitute error")));
            mem::swap(guard.as_deref_mut().unwrap(), &mut result);
            result
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_create_blog_success() {
        let blog_id = Uuid::new_v4();
        let service = MockBlogService {
            create_blog_result: Arc::new(std::sync::Mutex::new(Ok(Blog))),
        };
        let state = axum::extract::State(AppState {
            blog_service: Arc::new(service),
        });
        let body = axum::extract::Json(CreateBlogHttpRequestBody {
            name: "test".to_string(),
            email_address: "test".to_string(),
        });
        let expected = ApiSuccess::new(
            StatusCode::CREATED,
            CreateBlogResponseData {
                id: blog_id.to_string(),
            },
        );

        let actual = create_blog(state, body).await;
        assert!(
            actual.is_ok(),
            "expected create_blog to succeed, but got {:?}",
            actual
        );

        let actual = actual.unwrap();
        assert_eq!(
            actual, expected,
            "expected ApiSuccess {:?}, but got {:?}",
            expected, actual
        )
    }
}
