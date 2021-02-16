use crate::StrClass;
use ::smallvec::SmallVec;
use ::std::borrow::Cow;
use ::std::convert::From;
use ::std::fmt;
use ::std::ops::Add;

const ATTR_SMALL_VEC_SIZE: usize = 3;

#[derive(Clone, PartialEq, Debug)]
pub struct AttrClass<N> {
    pub(crate) parent: N,
    pub(crate) attrs: SmallVec<[&'static str; ATTR_SMALL_VEC_SIZE]>,
}

impl<N: Sized + fmt::Display> AttrClass<N> {
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

impl<N> Add<&'static str> for AttrClass<N> {
    type Output = StrClass<Self>;

    fn add(self, other: &'static str) -> Self::Output {
        StrClass {
            left: self,
            right: other,
        }
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
    use crate::Class;

    #[test]
    fn is_should_set_attr_if_is_set() {
        let attr = Class::new("mr-component").attr("large");
        assert_eq!(
            "mr-component mr-component--large mr-component--red",
            attr.maybe_attr("red", true).to_string()
        )
    }

    #[test]
    fn is_should_not_set_attr_if_is_set_is_false() {
        let attr = Class::new("mr-component").attr("large");
        assert_eq!(
            "mr-component mr-component--large",
            attr.maybe_attr("red", false).to_string()
        )
    }

    #[test]
    fn is_should_still_set_more_attr_after_false_maybe_attr() {
        let attr = Class::new("mr-component").attr("large");
        assert_eq!(
            "mr-component mr-component--large mr-component--blue",
            attr.maybe_attr("red", false).attr("blue").to_string()
        )
    }
}
