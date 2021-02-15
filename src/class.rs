use crate::{AttrClass, ElClass, StrClass};
use ::std::fmt;
use ::std::ops::Add;
use ::std::convert::From;
use ::std::borrow::Cow;

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
        AttrClass::new(self, attr)
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
