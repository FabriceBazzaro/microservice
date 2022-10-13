use crate::injectable;
use crate::injection::Component;
use std::fmt::Debug;

/// Constant component
///
#[derive(Debug)]
#[injectable(Component)]
pub struct Constant<T, const N: u64> where T: Debug + 'static {
    pub value: T
}

impl<T, const N: u64> Constant<T, N> where T: Debug + 'static {
    pub fn new(value: T) -> Self {
        Self {
            value
        }
    }
}
