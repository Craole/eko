use unicode_segmentation::UnicodeSegmentation;
use crate::commands::case::types::boundry::Boundary;

// #![allow(dead_code)]

pub fn split<T>(s: T, boundaries: &[Boundary]) -> Vec<String>
where
    T: AsRef<str>,
{
    use std::iter::once;

    let s = s.as_ref();
    let left_iter = s.graphemes(true);
    let mid_iter = s.graphemes(true).skip(1);
    let right_iter = s.graphemes(true).skip(2);

    let singles = left_iter.clone();
    let doubles = left_iter.clone().zip(mid_iter.clone());
    let triples = left_iter.zip(mid_iter).zip(right_iter);

    let singles = singles
        .map(|c| boundaries.iter().any(|b| b.detect_one(c)))
        .map(|split| if split { Some(true) } else { None });
    let doubles = doubles
        .map(|(c, d)| boundaries.iter().any(|b| b.detect_two(c, d)))
        .map(|split| if split { Some(false) } else { None });
    let triples = triples
        .map(|((c, d), e)| boundaries.iter().any(|b| b.detect_three(c, d, e)))
        .map(|split| if split { Some(false) } else { None });

    let split_points = singles
        .zip(once(None).chain(doubles))
        .zip(once(None).chain(triples).chain(once(None)))
        .map(|((s, d), t)| s.or(d).or(t));

    let mut words = Vec::new();
    let mut word = String::new();
    for (c, split) in s.graphemes(true).zip(split_points) {
        match split {
            // no split here
            None => word.push_str(c),
            // split here, consume letter
            Some(true) => words.push(std::mem::take(&mut word)),
            // split here, keep letter
            Some(false) => {
                words.push(std::mem::take(&mut word));
                word.push_str(c);
            }
        }
    }
    words.push(word);
    words.into_iter().filter(|s| !s.is_empty()).collect()
}
