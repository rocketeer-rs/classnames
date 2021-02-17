use crate::classes::DuoClass;
use crate::classes::OptionClass;
use crate::Class;
use ::smallvec::SmallVec;
use ::std::borrow::Cow;
use ::std::convert::From;
use ::std::fmt;
use ::std::ops::Add;

const ATTR_SMALL_VEC_SIZE: usize = 3;

#[derive(Clone, PartialEq, Debug)]
pub struct AttrClass<N> {
    parent: N,
    attrs: SmallVec<[&'static str; ATTR_SMALL_VEC_SIZE]>,
}

impl<N> Class for AttrClass<N> where N: fmt::Display + Sized + PartialEq + Clone {}

impl<N: fmt::Display + Sized> AttrClass<N> {
    pub(crate) fn new(parent: N) -> Self {
        Self {
            parent,
            attrs: SmallVec::new(),
        }
    }

    pub fn attr(self, attr: &'static str) -> Self {
        let mut attrs = self.attrs;
        attrs.push(attr);

        AttrClass {
            parent: self.parent,
            attrs: attrs,
        }
    }

    pub fn maybe_attr(self, attr: &'static str, is_set: bool) -> Self {
        if is_set {
            self.attr(attr)
        } else {
            self
        }
    }
}

impl<N, O> Add<O> for AttrClass<N>
where
    N: Class,
    O: Class,
{
    type Output = DuoClass<Self, O>;

    fn add(self, other: O) -> Self::Output {
        DuoClass::new(self, other)
    }
}

impl<'s, N> Add<&'s str> for AttrClass<N>
where
    N: Class,
{
    type Output = DuoClass<Self, &'s str>;

    fn add(self, other: &'s str) -> Self::Output {
        DuoClass::new(self, other)
    }
}

impl<N, O> Add<Option<O>> for AttrClass<N>
where
    N: Class,
    O: Class,
{
    type Output = DuoClass<Self, OptionClass<O>>;

    fn add(self, other: Option<O>) -> Self::Output {
        DuoClass::new(self, OptionClass::new(other))
    }
}

impl<N: fmt::Display> fmt::Display for AttrClass<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.parent)?;

        for attr in &self.attrs {
            write!(f, " {}--{}", self.parent, attr)?;
        }

        Ok(())
    }
}

impl<'a, N: fmt::Display> From<AttrClass<N>> for Cow<'a, str> {
    fn from(class: AttrClass<N>) -> Self {
        class.to_string().into()
    }
}

impl<'a, N: fmt::Display> From<AttrClass<N>> for String {
    fn from(class: AttrClass<N>) -> Self {
        class.to_string()
    }
}

#[cfg(test)]
mod maybe_attr {
    use crate::*;

    #[test]
    fn is_should_set_attr_if_is_set() {
        let attr = classname("mr-component").attr("large");
        assert_eq!(
            "mr-component mr-component--large mr-component--red",
            attr.maybe_attr("red", true).to_string()
        )
    }

    #[test]
    fn is_should_not_set_attr_if_is_set_is_false() {
        let attr = classname("mr-component").attr("large");
        assert_eq!(
            "mr-component mr-component--large",
            attr.maybe_attr("red", false).to_string()
        )
    }

    #[test]
    fn is_should_still_set_more_attr_after_false_maybe_attr() {
        let attr = classname("mr-component").attr("large");
        assert_eq!(
            "mr-component mr-component--large mr-component--blue",
            attr.maybe_attr("red", false).attr("blue").to_string()
        )
    }
}
