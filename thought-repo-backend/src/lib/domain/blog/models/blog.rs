use thiserror::Error;

pub struct Blog;

pub struct CreateBlogRequest;

#[derive(Debug, Error)]
pub enum CreateBlogError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
