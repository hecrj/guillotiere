#[cfg(feature = "serialization")]
#[macro_use]
pub extern crate serde;
pub extern crate euclid;

mod allocator;

pub use crate::allocator::*;

pub struct DeviceSpace;
pub type DeviceIntRect = euclid::TypedRect<i32, DeviceSpace>;
pub type DeviceIntPoint = euclid::TypedPoint2D<i32, DeviceSpace>;
pub type DeviceIntSize = euclid::TypedSize2D<i32, DeviceSpace>;
pub use euclid::size2;

