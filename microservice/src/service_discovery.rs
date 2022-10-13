use async_trait::async_trait;

use crate::error::*;
use crate::service::Service;
use crate::injectable;

#[async_trait]
#[injectable(Service)]
pub trait ServiceDiscovery: Service {
    async fn register(&self) -> Result<()>;
    async fn deregister(&self) -> Result<()>;
    async fn is_registered(&self) -> bool;
}

impl<T> Service for T where T: ServiceDiscovery {}
