use crate::classes::BaseClass;
use ::std::fmt;

pub trait Class: Sized + Clone + PartialEq + fmt::Display {}

/// Creates a new class.
pub fn classname(name: &'static str) -> BaseClass {
    BaseClass::new(name)
}
