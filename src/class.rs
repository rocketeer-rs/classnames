use crate::{AttrClass, ElClass, StrClass};
use ::std::borrow::Cow;
use ::std::convert::From;
use ::std::fmt;
use ::std::ops::Add;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Class {
    pub(crate) class: &'static str,
}

impl Class {
    pub fn new(base: &'static str) -> Self {
        Self { class: base }
    }

    pub fn class(&self) -> &'static str {
        self.class
    }

    pub fn el(self, class: &'static str) -> ElClass<Self> {
        ElClass {
            parent: self,
            class,
        }
    }

    pub fn attr(self, attr: &'static str) -> AttrClass<Self> {
        AttrClass::new(self).attr(attr)
    }

    pub fn maybe_attr(self, attr: &'static str, is_set: bool) -> AttrClass<Self> {
        AttrClass::new(self).maybe_attr(attr, is_set)
    }
}

impl Add<&'static str> for Class {
    type Output = StrClass<Self>;

    fn add(self, other: &'static str) -> Self::Output {
        StrClass {
            left: self,
            right: other,
        }
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.class)
    }
}

impl<'a> From<Class> for Cow<'a, str> {
    fn from(class: Class) -> Self {
        class.class().into()
    }
}

impl<'a> From<Class> for &'static str {
    fn from(class: Class) -> Self {
        class.class()
    }
}

impl From<Class> for String {
    fn from(class: Class) -> Self {
        class.class().to_string()
    }
}

#[cfg(test)]
mod maybe_attr {
    use crate::Class;

    #[test]
    fn is_should_set_attr_if_is_set() {
        let class = Class::new("mr-component").maybe_attr("red", true);
        assert_eq!("mr-component mr-component--red", class.to_string())
    }

    #[test]
    fn is_should_not_set_attr_if_is_set_is_false() {
        let class = Class::new("mr-component").maybe_attr("red", false);
        assert_eq!("mr-component", class.to_string())
    }

    #[test]
    fn is_should_still_set_more_attr_after_false_maybe_attr() {
        let class = Class::new("mr-component")
            .maybe_attr("red", false)
            .attr("blue");
        assert_eq!("mr-component mr-component--blue", class.to_string())
    }
}
