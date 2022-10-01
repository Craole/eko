#![allow(dead_code)]
use std::iter;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum WordCase {
    Lower,
    Upper,
    Capital,
    Toggle,
}

impl WordCase {
    fn mutate(&self, word: &str) -> String {
        use WordCase::*;
        match self {
            Lower => word.to_lowercase(),
            Upper => word.to_uppercase(),
            Capital => {
                let mut chars = word.chars();
                if let Some(c) = chars.next() {
                    c.to_uppercase()
                        .chain(chars.as_str().to_lowercase().chars())
                        .collect()
                } else {
                    String::new()
                }
            }
            Toggle => {
                let mut chars = word.chars();
                if let Some(c) = chars.next() {
                    c.to_lowercase()
                        .chain(chars.as_str().to_uppercase().chars())
                        .collect()
                } else {
                    String::new()
                }
            }
        }
    }
}

/// A pattern is how a set of words is mutated before joining with
/// a delimeter.
///
/// The `Random` and `PseudoRandom` patterns are used for their respective cases
/// and are only available in the "random" feature.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Pattern {
    /// Lowercase patterns make all words lowercase.
    /// ```
    /// use convert_case::Pattern;
    /// assert_eq!(
    ///     vec!["case", "conversion", "library"],
    ///     Pattern::Lowercase.mutate(&["Case", "CONVERSION", "library"])
    /// );
    /// ```
    Lowercase,

    /// Uppercase patterns make all words uppercase.
    /// ```
    /// use convert_case::Pattern;
    /// assert_eq!(
    ///     vec!["CASE", "CONVERSION", "LIBRARY"],
    ///     Pattern::Uppercase.mutate(&["Case", "CONVERSION", "library"])
    /// );
    /// ```
    Uppercase,

    /// Capital patterns makes the first letter of each word uppercase
    /// and the remaining letters of each word lowercase.
    /// ```
    /// use convert_case::Pattern;
    /// assert_eq!(
    ///     vec!["Case", "Conversion", "Library"],
    ///     Pattern::Capital.mutate(&["Case", "CONVERSION", "library"])
    /// );
    /// ```
    Capital,

    /// Capital patterns make the first word capitalized and the
    /// remaining lowercase.
    /// ```
    /// use convert_case::Pattern;
    /// assert_eq!(
    ///     vec!["Case", "conversion", "library"],
    ///     Pattern::Sentence.mutate(&["Case", "CONVERSION", "library"])
    /// );
    /// ```
    Sentence,

    /// Camel patterns make the first word lowercase and the remaining
    /// capitalized.
    /// ```
    /// use convert_case::Pattern;
    /// assert_eq!(
    ///     vec!["case", "Conversion", "Library"],
    ///     Pattern::Camel.mutate(&["Case", "CONVERSION", "library"])
    /// );
    /// ```
    Camel,

    /// Alternating patterns make each letter of each word alternate
    /// between lowercase and uppercase.  They alternate across words,
    /// which means the last letter of one word and the first letter of the
    /// next will not be the same letter casing.
    /// ```
    /// use convert_case::Pattern;
    /// assert_eq!(
    ///     vec!["cAsE", "cOnVeRsIoN", "lIbRaRy"],
    ///     Pattern::Alternating.mutate(&["Case", "CONVERSION", "library"])
    /// );
    /// assert_eq!(
    ///     vec!["aNoThEr", "ExAmPlE"],
    ///     Pattern::Alternating.mutate(&["Another", "Example"]),
    /// );
    /// ```
    Alternating,

    /// Toggle patterns have the first letter of each word uppercase
    /// and the remaining letters of each word uppercase.
    /// ```
    /// use convert_case::Pattern;
    /// assert_eq!(
    ///     vec!["cASE", "cONVERSION", "lIBRARY"],
    ///     Pattern::Toggle.mutate(&["Case", "CONVERSION", "library"])
    /// );
    /// ```
    Toggle,
}

impl Pattern {
    /// Generates a vector of new `String`s in the right pattern given
    /// the input strings.
    /// ```
    /// use convert_case::Pattern;
    ///
    /// assert_eq!(
    ///     vec!["crack", "the", "skye"],
    ///     Pattern::Lowercase.mutate(&vec!["CRACK", "the", "Skye"]),
    /// )
    /// ```
    pub fn mutate(&self, words: &[&str]) -> Vec<String> {
        use Pattern::*;
        match self {
            Lowercase => words
                .iter()
                .map(|word| WordCase::Lower.mutate(word))
                .collect(),
            Uppercase => words
                .iter()
                .map(|word| WordCase::Upper.mutate(word))
                .collect(),
            Capital => words
                .iter()
                .map(|word| WordCase::Capital.mutate(word))
                .collect(),
            Toggle => words
                .iter()
                .map(|word| WordCase::Toggle.mutate(word))
                .collect(),
            Sentence => {
                let word_cases =
                    iter::once(WordCase::Capital).chain(iter::once(WordCase::Lower).cycle());
                words
                    .iter()
                    .zip(word_cases)
                    .map(|(word, word_case)| word_case.mutate(word))
                    .collect()
            }
            Camel => {
                let word_cases =
                    iter::once(WordCase::Lower).chain(iter::once(WordCase::Capital).cycle());
                words
                    .iter()
                    .zip(word_cases)
                    .map(|(word, word_case)| word_case.mutate(word))
                    .collect()
            }
            Alternating => alternating(words),
        }
    }
}

fn alternating(words: &[&str]) -> Vec<String> {
    let mut upper = false;
    words
        .iter()
        .map(|word| {
            word.chars()
                .map(|letter| {
                    if letter.is_uppercase() || letter.is_lowercase() {
                        if upper {
                            upper = false;
                            letter.to_uppercase().to_string()
                        } else {
                            upper = true;
                            letter.to_lowercase().to_string()
                        }
                    } else {
                        letter.to_string()
                    }
                })
                .collect()
        })
        .collect()
}