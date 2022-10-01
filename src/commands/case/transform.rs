#![allow(dead_code)]

use super::{types::{boundry::Boundary, conversion::Converter}, case::Case};
// use super::{case::Case, boundry::Boundary, conversion::Converter};

pub trait Modifiable<T: AsRef<str>> {
    fn to_case(&self, case: Case) -> String;

    #[allow(clippy::wrong_self_convention)]
    fn from_case(&self, case: Case) -> StateConverter<T>;
    fn with_boundaries(&self, bs: &[Boundary]) -> StateConverter<T>;
    fn is_case(&self, case: Case) -> bool;
}

impl<T: AsRef<str>> Modifiable<T> for T
where
    String: PartialEq<T>,
{
    fn to_case(&self, case: Case) -> String {
        StateConverter::new(self).to_case(case)
    }

    fn with_boundaries(&self, bs: &[Boundary]) -> StateConverter<T> {
        StateConverter::new(self).with_boundaries(bs)
    }

    fn from_case(&self, case: Case) -> StateConverter<T> {
        StateConverter::new_from_case(self, case)
    }

    fn is_case(&self, case: Case) -> bool {
        &self.to_case(case) == self
    }
}

pub struct StateConverter<'a, T: AsRef<str>> {
    s: &'a T,
    conv: Converter,
}

impl<'a, T: AsRef<str>> StateConverter<'a, T> {
    /// Only called by Modifiable function to_case()
    fn new(s: &'a T) -> Self {
        Self {
            s,
            conv: Converter::new(),
        }
    }

    /// Only called by Modifiable function from_case()
    fn new_from_case(s: &'a T, case: Case) -> Self {
        Self {
            s,
            conv: Converter::new().from_case(case),
        }
    }

    pub fn from_case(self, case: Case) -> Self {
        Self {
            conv: self.conv.from_case(case),
            ..self
        }
    }

    pub fn with_boundaries(self, bs: &[Boundary]) -> Self {
        Self {
            s: self.s,
            conv: self.conv.set_boundaries(bs),
        }
    }

    pub fn without_boundaries(self, bs: &[Boundary]) -> Self {
        Self {
            s: self.s,
            conv: self.conv.remove_boundaries(bs),
        }
    }

    pub fn to_case(self, case: Case) -> String {
        self.conv.to_case(case).convert(self.s)
    }
}
