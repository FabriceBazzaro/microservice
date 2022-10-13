use crate::injection::Component;
use crate::injectable;

#[injectable(Component)]
pub trait Service: Component {}
