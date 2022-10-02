#![allow(dead_code)]

use super::{pattern::Pattern, boundry::Boundary};

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum Case {
    Upper,
    Lower,
    Title,
    Toggle,
    Camel,
    Pascal,
    UpperCamel,
    Snake,
    UpperSnake,
    ScreamingSnake,
    Kebab,
    Cobol,
    UpperKebab,
    Train,
    Flat,
    UpperFlat,
    Alternating,
}

impl Case {
    pub const fn delim(&self) -> &'static str {
        use Case::*;
        match self {
            Upper | Lower | Title | Toggle | Alternating => " ",
            Snake | UpperSnake | ScreamingSnake => "_",
            Kebab | Cobol | UpperKebab | Train => "-",
            UpperFlat | Flat | Camel | UpperCamel | Pascal => "",
        }
    }

    pub const fn pattern(&self) -> Pattern {
        use Case::*;
        match self {
            Upper | UpperSnake | ScreamingSnake | UpperFlat | Cobol | UpperKebab => {
                Pattern::Uppercase
            }
            Lower | Snake | Kebab | Flat => Pattern::Lowercase,
            Title | Pascal | UpperCamel | Train => Pattern::Capital,
            Camel => Pattern::Camel,
            Toggle => Pattern::Toggle,
            Alternating => Pattern::Alternating,
        }
    }

    pub fn boundaries(&self) -> Vec<Boundary> {
        use Boundary::*;
        use Case::*;
        match self {
            Upper | Lower | Title | Toggle | Alternating => vec![Space],
            Snake | UpperSnake | ScreamingSnake => vec![Underscore],
            Kebab | Cobol | UpperKebab | Train => vec![Hyphen],
            UpperFlat | Flat => vec![],
            Camel | UpperCamel | Pascal => vec![
                LowerUpper, Acronym, LowerDigit, UpperDigit, DigitLower, DigitUpper,
            ],
        }
    }

    pub fn cases() -> Vec<Case> {
        use Case::*;
        vec![
            Upper,
            Lower,
            Title,
            Toggle,
            Camel,
            Pascal,
            UpperCamel,
            Snake,
            UpperSnake,
            ScreamingSnake,
            Kebab,
            Cobol,
            UpperKebab,
            Train,
            Flat,
            UpperFlat,
            Alternating,
        ]
    }
}
