use std::any::{Any, TypeId, type_name};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::fmt::Debug;
use injection_macros::*;

use super::error::*;
use super::Err;

/// Errors generated using a Registry
#[derive(Error, PartialEq, Debug)]
pub enum RegistryError<'a> {
    #[error("Multiple components correspond to dependency {name}, inject dependencies manually")]
    MultipleComponentsError { name: &'a str },

    #[error("No component correspond to dependency {name}")]
    NoComponentError { name: &'a str },

    #[error("Component type error during conversion")]
    ComponentTypeError,

    #[error("Component new with injection is not implemented and must be called manually: use #[inject]")]
    NotImplemented
}

/// If we want to use a clean architecture of the object classes, Component is the Base trait at the root
/// of each injectable objects which can be registered in a Registry
#[injectable]
pub trait Component {
    fn register(component_ref: Arc<Mutex<Self>>, registry: &mut Registry) where Self: Sized + 'static;
    fn struct_impl_trait<T>() -> bool where T: ?Sized + 'static, Self: Sized + 'static;
    fn is_impl_trait<T>(&self) -> bool where T: ?Sized + 'static, Self: Sized + 'static {
         Self::struct_impl_trait::<T>()
    }
}

/// Injection is the implementation of the constructor of the class which is automatically called when dependency injection is used
pub trait Injection {
    fn new_from_reg(_registry: &mut Registry) -> Result<Self> where Self: Sized {
        Err!(RegistryError::NotImplemented)
    }
}

/// Registry management vector
pub trait RegistryVec {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}


impl<T> RegistryVec for Mutex<Vec<Arc<Mutex<T>>>> where T: ?Sized + 'static {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }
}


pub struct Registry {
    registries: HashMap<TypeId, Box<dyn RegistryVec>>
}

impl Registry {
    pub fn new() -> Self {
        Self {
            registries: HashMap::new()
        }
    }

    pub fn register_instance<T>(&mut self, component: T) ->  Arc<Mutex<T>> where T: Component + 'static {
        let component: Arc<Mutex<T>> = Arc::new(Mutex::new(component));
        T::register(component.clone(), self);
        component
    }

    pub fn register_with_type<T>(&mut self, component: Arc<Mutex<T>>) where T: ?Sized + 'static {
        let id = TypeId::of::<T>();
        if let Some(component_vec) = self.registries.get_mut(&id) {
            if let Some(registry_vec) = component_vec.as_any_mut().downcast_mut::<Mutex<Vec<Arc<Mutex<T>>>>>() {
                registry_vec.lock().unwrap().push(component);
            }
            else {
                println!("Error during conversion");
            }
        }
        else {
            let mut component_vec: Vec<Arc<Mutex<T>>> = Vec::new();
            component_vec.push(component);
            self.registries.insert(id, Box::new(Mutex::new(component_vec)));
        }
    }


    pub fn register<T>(&mut self) -> Result<Arc<Mutex<T>>> where T: Component + Injection + 'static {
        let component: Arc<Mutex<T>> = Arc::new(Mutex::new(T::new_from_reg(self)?));
        T::register(component.clone(), self);
        Ok(component)
    }

    pub fn get<T>(&mut self) -> Result<Arc<Mutex<T>>> where T: ?Sized + 'static {
        let id = TypeId::of::<T>();
        if let Some(registry_entry) = self.registries.get_mut(&id) {
            if let Some(registry_vec) = registry_entry.as_any_mut().downcast_mut::<Mutex<Vec<Arc<Mutex<T>>>>>() {
                let len = registry_vec.lock().unwrap().len();
                match len {
                    0 => {
                        Err!(RegistryError::NoComponentError { name: type_name::<T>() })
                    },
                    1 => {
                        Ok(registry_vec.lock().unwrap()[0].clone())
                    },
                    _ => {
                        Err!(RegistryError::MultipleComponentsError { name: type_name::<T>() })
                    }
                }
            }
            else {
                Err!(RegistryError::ComponentTypeError)
            }
        }
        else {
            Err!(RegistryError::NoComponentError { name: type_name::<T>() })
        }
    }
}

impl Drop for Registry {
    fn drop(&mut self) {
        println!("Drop registry memory");
        self.registries.clear();
    }
}
