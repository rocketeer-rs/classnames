use crate::classes::{AttrClass, DuoClass, ElClass, OptionClass};
use crate::Class;
use ::std::borrow::Cow;
use ::std::convert::From;
use ::std::fmt;
use ::std::ops::Add;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct BaseClass {
    class: &'static str,
}

impl Class for BaseClass {}

impl BaseClass {
    pub fn new(base: &'static str) -> Self {
        Self { class: base }
    }

    pub fn class(&self) -> &'static str {
        self.class
    }

    pub fn el<'a>(self, class: &'a str) -> ElClass<Self, &'a str> {
        ElClass::new(self, class)
    }

    pub fn attr(self, attr: &'static str) -> AttrClass<Self> {
        AttrClass::new(self).attr(attr)
    }

    pub fn maybe_attr(self, attr: &'static str, is_set: bool) -> AttrClass<Self> {
        AttrClass::new(self).maybe_attr(attr, is_set)
    }
}

impl<'s> Add<&'s str> for BaseClass {
    type Output = DuoClass<Self, &'s str>;

    fn add(self, other: &'s str) -> Self::Output {
        DuoClass::new(self, other)
    }
}

impl<O> Add<Option<O>> for BaseClass
where
    O: Class,
{
    type Output = DuoClass<Self, OptionClass<O>>;

    fn add(self, other: Option<O>) -> Self::Output {
        DuoClass::new(self, OptionClass::new(other))
    }
}

impl<O> Add<O> for BaseClass
where
    O: Class,
{
    type Output = DuoClass<Self, O>;

    fn add(self, other: O) -> Self::Output {
        DuoClass::new(self, other)
    }
}

impl fmt::Display for BaseClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.class)
    }
}

impl<'a> From<BaseClass> for Cow<'a, str> {
    fn from(class: BaseClass) -> Self {
        class.class().into()
    }
}

impl<'a> From<BaseClass> for &'static str {
    fn from(class: BaseClass) -> Self {
        class.class()
    }
}

impl From<BaseClass> for String {
    fn from(class: BaseClass) -> Self {
        class.class().to_string()
    }
}

#[cfg(test)]
mod maybe_attr {
    use super::*;

    #[test]
    fn is_should_set_attr_if_is_set() {
        let class = BaseClass::new("mr-component").maybe_attr("red", true);
        assert_eq!("mr-component mr-component--red", class.to_string())
    }

    #[test]
    fn is_should_not_set_attr_if_is_set_is_false() {
        let class = BaseClass::new("mr-component").maybe_attr("red", false);
        assert_eq!("mr-component", class.to_string())
    }

    #[test]
    fn is_should_still_set_more_attr_after_false_maybe_attr() {
        let class = BaseClass::new("mr-component")
            .maybe_attr("red", false)
            .attr("blue");
        assert_eq!("mr-component mr-component--blue", class.to_string())
    }
}
