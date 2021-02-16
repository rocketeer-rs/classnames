use crate::{AttrClass, StrClass};
use ::std::borrow::Cow;
use ::std::convert::From;
use ::std::fmt;
use ::std::ops::Add;

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
        AttrClass::new(self).attr(attr)
    }

    pub fn maybe_attr(self, attr: &'static str, is_set: bool) -> AttrClass<Self> {
        AttrClass::new(self).maybe_attr(attr, is_set)
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

#[cfg(test)]
mod maybe_attr {
    use crate::Class;

    #[test]
    fn is_should_set_attr_if_is_set() {
        let el = Class::new("mr-component").el("child");
        assert_eq!(
            "mr-component__child mr-component__child--red",
            el.maybe_attr("red", true).to_string()
        )
    }

    #[test]
    fn is_should_not_set_attr_if_is_set_is_false() {
        let el = Class::new("mr-component").el("child");
        assert_eq!(
            "mr-component__child",
            el.maybe_attr("red", false).to_string()
        )
    }

    #[test]
    fn is_should_still_set_more_attr_after_false_maybe_attr() {
        let el = Class::new("mr-component").el("child");
        assert_eq!(
            "mr-component__child mr-component__child--blue",
            el.maybe_attr("red", false).attr("blue").to_string()
        )
    }
}
