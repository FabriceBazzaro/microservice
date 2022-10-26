use async_trait::async_trait;

use crate::error::*;
use crate::service::Service;
use crate::share::SimpleUrl;
use crate::injectable;

#[async_trait]
#[injectable(Service)]
pub trait ServiceDiscovery: Service {
    async fn register(&self) -> Result<()>;
    async fn deregister(&self) -> Result<()>;
    async fn is_registered(&self) -> Result<bool>;
    async fn get_service_url(&self, service_name: String) -> Result<SimpleUrl>;

}

impl<T> Service for T where T: ServiceDiscovery {}
