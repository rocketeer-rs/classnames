use crate::Class;
use crate::classes::OptionClass;
use ::std::borrow::Cow;
use ::std::convert::From;
use ::std::fmt;
use ::std::ops::Add;

#[derive(Clone, PartialEq, Debug)]
pub struct DuoClass<L, R> {
    left: L,
    right: R,
}

impl<L, R> Class for DuoClass<L, R>
where
    L: fmt::Display + Sized + PartialEq + Clone,
    R: fmt::Display + Sized + PartialEq + Clone,
{}

impl<L, R> DuoClass<L, R>
where
    L: fmt::Display + Sized,
    R: fmt::Display + Sized,
 {
    pub(crate) fn new(
        left: L,
        right: R,
    ) -> Self {
        Self {
            left,
            right,
        }
    }
}

impl<'s, L, R> Add<&'s str> for DuoClass<L, R>
where
    L: fmt::Display + Sized,
    R: fmt::Display + Sized,
{
    type Output = DuoClass<Self, &'s str>;

    fn add(self, other: &'s str) -> Self::Output {
        DuoClass::new(self, other)
    }
}

impl<L, R, O> Add<O> for DuoClass<L, R>
where
    L: fmt::Display + Sized,
    R: fmt::Display + Sized,
    O: Class,
{
    type Output = DuoClass<Self, O>;

    fn add(self, other: O) -> Self::Output {
        DuoClass::new(self, other)
    }
}

impl<L, R, O> Add<Option<O>> for DuoClass<L, R>
where
    L: fmt::Display + Sized,
    R: fmt::Display + Sized,
    O: Class,
{
    type Output = DuoClass<Self, OptionClass<O>>;

    fn add(self, other: Option<O>) -> Self::Output {
        DuoClass::new(self, OptionClass::new(other))
    }
}

impl<L: fmt::Display, R: fmt::Display> fmt::Display for DuoClass<L, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.left, self.right)
    }
}

impl<'a, L: fmt::Display, R: fmt::Display> From<DuoClass<L, R>> for Cow<'a, str> {
    fn from(class: DuoClass<L, R>) -> Self {
        class.to_string().into()
    }
}

impl<'a, L: fmt::Display, R: fmt::Display> From<DuoClass<L, R>> for String {
    fn from(class: DuoClass<L, R>) -> Self {
        class.to_string()
    }
}
