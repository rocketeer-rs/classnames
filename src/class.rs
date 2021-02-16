use crate::classes::{BaseClass};
use ::std::fmt;

pub trait Class : Sized + Clone + PartialEq + fmt::Display {
}

pub fn classname(base: &'static str) -> BaseClass {
    BaseClass::new(base)
}

#[cfg(test)]
mod maybe_attr {
    use super::*;

    #[test]
    fn is_should_set_attr_if_is_set() {
        let class = classname("mr-component").maybe_attr("red", true);
        assert_eq!("mr-component mr-component--red", class.to_string())
    }

    #[test]
    fn is_should_not_set_attr_if_is_set_is_false() {
        let class = classname("mr-component").maybe_attr("red", false);
        assert_eq!("mr-component", class.to_string())
    }

    #[test]
    fn is_should_still_set_more_attr_after_false_maybe_attr() {
        let class = classname("mr-component")
            .maybe_attr("red", false)
            .attr("blue");
        assert_eq!("mr-component mr-component--blue", class.to_string())
    }
}
