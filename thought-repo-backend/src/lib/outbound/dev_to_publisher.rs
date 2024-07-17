use crate::domain::blog::ports::BlogPublisher;

#[derive(Clone)]
pub struct DevToPublisher;

impl BlogPublisher for DevToPublisher {
    async fn publish_to_dev_to(&self) -> () {}
}
