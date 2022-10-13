use async_trait::async_trait;
use bytes::Bytes;
use futures::stream::Stream;
use std::pin::Pin;

use crate::error::*;
use crate::service::Service;
use crate::injectable;

pub trait Subscriber: Stream<Item = Bytes> where Self::Item: Clone {}

#[async_trait]
#[injectable(Service)]
pub trait PubSub: Service + Send + Sync {
    async fn publish(&self, subject: String, payload: Vec<u8>) -> Result<()>;
    async fn subscribe(&self, subject: String) -> Result<Pin<Box<dyn Subscriber>>>;
}
