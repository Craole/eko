#![allow(dead_code)]
use unicode_segmentation::UnicodeSegmentation;

use crate::functions::case::utilities::check_char::{is_digit, is_lowercase, is_uppercase};

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Boundary {
    Hyphen,
    Underscore,
    Space,
    UpperLower,
    LowerUpper,
    DigitUpper,
    UpperDigit,
    DigitLower,
    LowerDigit,
    Acronym,
}

impl Boundary {
    pub fn list_from(s: &str) -> Vec<Self> {
        Boundary::all()
            .iter()
            .filter(|boundary| {
                let left_iter = s.graphemes(true);
                let mid_iter = s.graphemes(true).skip(1);
                let right_iter = s.graphemes(true).skip(2);

                let mut one_iter = left_iter.clone();

                // Also capture when the previous pair was both uppercase, so we don't
                // match the UpperLower boundary in the case of Acronym
                let two_iter = left_iter.clone().zip(mid_iter.clone());
                let mut two_iter_and_upper = two_iter.clone().zip(
                    std::iter::once(false)
                        .chain(two_iter.map(|(a, b)| is_uppercase(a) && is_uppercase(b))),
                );

                let mut three_iter = left_iter.zip(mid_iter).zip(right_iter);

                one_iter.any(|a| boundary.detect_one(a))
                    || two_iter_and_upper
                        .any(|((a, b), is_acro)| boundary.detect_two(a, b) && !is_acro)
                    || three_iter.any(|((a, b), c)| boundary.detect_three(a, b, c))
            })
            .copied()
            .collect()
    }

    pub fn defaults() -> Vec<Self> {
        use Boundary::*;
        vec![
            Underscore, Hyphen, Space, LowerUpper, UpperDigit, DigitUpper, DigitLower, LowerDigit,
            Acronym,
        ]
    }

    pub fn delims() -> Vec<Self> {
        use Boundary::*;
        vec![Hyphen, Underscore, Space]
    }

    pub fn digits() -> Vec<Self> {
        use Boundary::*;
        vec![DigitUpper, UpperDigit, DigitLower, LowerDigit]
    }

    pub fn letter_digit() -> Vec<Self> {
        use Boundary::*;
        vec![UpperDigit, LowerDigit]
    }

    pub fn digit_letter() -> Vec<Self> {
        use Boundary::*;
        vec![DigitUpper, DigitLower]
    }

    pub fn all() -> Vec<Self> {
        use Boundary::*;
        vec![
            Hyphen, Underscore, Space, LowerUpper, UpperLower, DigitUpper, UpperDigit, DigitLower,
            LowerDigit, Acronym,
        ]
    }

    pub fn detect_one(&self, c: &str) -> bool {
        use Boundary::*;
        match self {
            Hyphen => c == "-",
            Underscore => c == "_",
            Space => c == " ",
            _ => false,
        }
    }

    pub fn detect_two(&self, c: &str, d: &str) -> bool {
        use Boundary::*;
        match self {
            UpperLower => is_uppercase(c) && is_lowercase(d),
            LowerUpper => is_lowercase(c) && is_uppercase(d),
            DigitUpper => is_digit(c) && is_uppercase(d),
            UpperDigit => is_uppercase(c) && is_digit(d),
            DigitLower => is_digit(c) && is_lowercase(d),
            LowerDigit => is_lowercase(c) && is_digit(d),
            _ => false,
        }
    }

    pub fn detect_three(&self, c: &str, d: &str, e: &str) -> bool {
        use Boundary::*;
        if let Acronym = self {
            is_uppercase(c) && is_uppercase(d) && is_lowercase(e)
        } else {
            false
        }
    }
}
