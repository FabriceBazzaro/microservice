use std::sync::{Arc, Mutex};

pub use injection_macros::*;
pub use tools_macros::*;

pub mod share;
pub mod error;
pub mod injection;
pub mod constant;
pub mod service;
pub mod config;
pub mod logger;
pub mod pubsub;
pub mod service_discovery;

pub struct Microservice {
    pub registry: injection::Registry
}

impl Microservice {
    pub fn new() -> Microservice {
        Microservice {
            registry: injection::Registry::new()
        }
    }

    pub fn register_instance<T>(&mut self, component: T) -> Arc<Mutex<T>> where T: injection::Component + 'static {
        self.registry.register_instance::<T>(component)
    }

    pub fn register<T>(&mut self) -> error::Result<Arc<Mutex<T>>> where T: injection::Component + injection::Injection + 'static {
        let result = self.registry.register::<T>()?;
        if T::struct_impl_trait::<dyn logger::Logger>() {
            let int_logger = self.get::<dyn logger::Logger>().unwrap();
            logger::register_logger(int_logger.clone());
        }
        Ok(result)
    }

    pub fn get<T>(&mut self) -> error::Result<Arc<Mutex<T>>> where T: ?Sized + 'static {
        self.registry.get::<T>()
    }
}

impl Drop for Microservice {
    fn drop(&mut self) {
        trace!("Drop microservice");
    }
}
