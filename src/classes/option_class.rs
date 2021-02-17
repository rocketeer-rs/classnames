use crate::classes::DuoClass;
use crate::Class;
use ::std::borrow::Cow;
use ::std::convert::From;
use ::std::fmt;
use ::std::ops::Add;

#[derive(Clone, PartialEq, Debug)]
pub struct OptionClass<C> {
    option: Option<C>,
}

impl<C> Class for OptionClass<C> where C: fmt::Display + Sized + PartialEq + Clone {}

impl<C> OptionClass<C>
where
    C: fmt::Display + Sized,
{
    pub(crate) fn new(option: Option<C>) -> Self {
        Self { option }
    }
}

impl<C, O> Add<O> for OptionClass<C>
where
    C: Class,
    O: Class,
{
    type Output = DuoClass<Self, O>;

    fn add(self, other: O) -> Self::Output {
        DuoClass::new(self, other)
    }
}

impl<'s, C> Add<&'s str> for OptionClass<C>
where
    C: Class,
{
    type Output = DuoClass<Self, &'s str>;

    fn add(self, other: &'s str) -> Self::Output {
        DuoClass::new(self, other)
    }
}

impl<C, O> Add<Option<O>> for OptionClass<C>
where
    C: Class,
    O: Class,
{
    type Output = DuoClass<Self, OptionClass<O>>;

    fn add(self, other: Option<O>) -> Self::Output {
        DuoClass::new(self, OptionClass::new(other))
    }
}

impl<C: fmt::Display> fmt::Display for OptionClass<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(class) = &self.option {
            write!(f, "{}", class)?;
        }

        Ok(())
    }
}

impl<'a, C: fmt::Display> From<OptionClass<C>> for Cow<'a, str> {
    fn from(class: OptionClass<C>) -> Self {
        class.to_string().into()
    }
}

impl<'a, C: fmt::Display> From<OptionClass<C>> for String {
    fn from(class: OptionClass<C>) -> Self {
        class.to_string()
    }
}

#[cfg(test)]
mod display {
    use super::*;
    use crate::classes::BaseClass;

    #[test]
    fn it_should_be_blank_if_none() {
        let o = OptionClass::<BaseClass>::new(None);
        assert_eq!(o.to_string(), "");
    }

    #[test]
    fn it_should_print_if_class_provided() {
        let o = OptionClass::<BaseClass>::new(Some(BaseClass::new("home-page")));
        assert_eq!(o.to_string(), "home-page");
    }
}
