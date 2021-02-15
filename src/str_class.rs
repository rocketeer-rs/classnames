use ::std::fmt;
use ::std::ops::Add;
use ::std::convert::From;
use ::std::borrow::Cow;

#[derive(Clone, PartialEq, Debug)]
pub struct StrClass<L> {
    pub(crate) left: L,
    pub(crate) right: &'static str,
}

impl<L> Add<&'static str> for StrClass<L> {
    type Output = StrClass<Self>;

    fn add(self, other: &'static str) -> Self::Output {
        StrClass {
            left: self,
            right: other,
        }
    }
}

impl<N: fmt::Display> fmt::Display for StrClass<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.left, self.right)
    }
}

impl<'a, N: fmt::Display> From<StrClass<N>> for Cow<'a, str> {
    fn from(class: StrClass<N>) -> Self {
        class.to_string().into()
    }
}

impl<'a, N: fmt::Display> From<StrClass<N>> for String {
    fn from(class: StrClass<N>) -> Self {
        class.to_string()
    }
}
