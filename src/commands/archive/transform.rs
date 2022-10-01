#![allow(dead_code)]

use crate::commands::case::types::{case::Case, modifiable::Modifiable};

// pub fn to_case(&self, case: Case) -> String;

pub fn case_lower(input: &str) {
    // Lowercase strings are delimited by spaces and all characters are lowercase.
    let input = String::from(input);
    println!("{}", input.to_case(Case::Lower));
}

pub fn case_lower_flat(input: &str) {
    // Flat case strings are all lowercase, with no delimiter. Converting to this case is lossy.
    println!("{}", input.to_case(Case::Flat));
}

pub fn case_upper(input: &String) {
    // Uppercase strings are delimited by spaces and all characters are uppercase.
    println!("{}", input.to_case(Case::Upper));
}

pub fn case_upper_flat(input: &String) {
    // Upper flat case strings are all uppercase, with no delimiter. Converting to this case is lossy.
    println!("{}", input.to_case(Case::UpperFlat));
}

pub fn case_title(input: &String) {
    // Title case strings are delimited by spaces. Only the leading character of each word is uppercase.
    println!("{}", input.to_case(Case::Title));
}

pub fn case_toggle(input: &String) {
    // Toggle case strings are delimited by spaces. All characters are uppercase except for the leading character of each word, which is lowercase.
    println!("{}", input.to_case(Case::Toggle));
}

pub fn case_camel(input: &String) {
    // Camel case strings are lowercase, but for every word except the first the first letter is capitalized.
    println!("{}", input.to_case(Case::Camel));
}

pub fn case_pascal(input: &String) {
    // Pascal case strings are lowercase, but for every word the first letter is capitalized.
    // Also referred to as Upper camel case.
    println!("{}", input.to_case(Case::Pascal));
}

pub fn case_snake(input: &String) {
    // Snake case strings are delimited by underscores _ and are all lowercase.
    println!("{}", input.to_case(Case::Snake));
}

pub fn case_snake_screaming(input: &String) {
    // Also referred to as Upper samel case.
    println!("{}", input.to_case(Case::ScreamingSnake));
}

pub fn case_kebab(input: &String) {
    // Kebab case strings are delimited by hyphens - and are all lowercase.
    println!("{}", input.to_case(Case::Kebab));
}

pub fn case_cobol(input: &String) {
    // Cobol case strings are delimited by hyphens - and are all uppercase.
    println!("{}", input.to_case(Case::Cobol));
}

pub fn case_train(input: &String) {
    // Train case strings are delimited by hyphens -. All characters are lowercase except for the leading character of each word.
    println!("{}", input.to_case(Case::Train));
}
