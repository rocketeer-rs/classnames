use crate::{AttrClass, StrClass};
use ::std::fmt;
use ::std::ops::Add;
use ::std::convert::From;
use ::std::borrow::Cow;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct ElClass<N> {
    pub(crate) parent: N,
    pub(crate) class: &'static str,
}

impl<N: Sized + fmt::Display> ElClass<N> {
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

impl<N> Add<&'static str> for ElClass<N> {
    type Output = StrClass<Self>;

    fn add(self, other: &'static str) -> Self::Output {
        StrClass {
            left: self,
            right: other,
        }
    }
}

impl<N: fmt::Display> fmt::Display for ElClass<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}__{}", self.parent, self.class)
    }
}

impl<'a, N: fmt::Display> From<ElClass<N>> for Cow<'a, str> {
    fn from(class: ElClass<N>) -> Self {
        class.to_string().into()
    }
}

impl<'a, N: fmt::Display> From<ElClass<N>> for String {
    fn from(class: ElClass<N>) -> Self {
        class.to_string()
    }
}
