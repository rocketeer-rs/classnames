use crate::StrClass;
use ::smallvec::SmallVec;
use ::std::fmt;
use ::std::ops::Add;
use ::std::convert::From;
use ::std::borrow::Cow;

const ATTR_SMALL_VEC_SIZE: usize = 3;

#[derive(Clone, PartialEq, Debug)]
pub struct AttrClass<N> {
    pub(crate) parent: N,
    pub(crate) attrs: SmallVec<[&'static str; ATTR_SMALL_VEC_SIZE]>,
}

impl<N: Sized + fmt::Display> AttrClass<N> {
    pub(crate) fn new(parent: N, attr: &'static str) -> Self {
        let mut attrs = SmallVec::new();
        attrs.push(attr);

        Self { parent, attrs }
    }

    pub fn attr(self, attr: &'static str) -> Self {
        let mut attrs = self.attrs;
        attrs.push(attr);

        AttrClass {
            parent: self.parent,
            attrs: attrs,
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
