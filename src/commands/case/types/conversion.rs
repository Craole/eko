#![allow(dead_code)]
use crate::commands::case::{utilities::segment::split, case::Case};

use super::{boundry::Boundary, pattern::Pattern};

pub struct Converter {
    pub boundaries: Vec<Boundary>,
    pub pattern: Option<Pattern>,
    pub delim: String,
}

impl Default for Converter {
    fn default() -> Self {
        Converter {
            boundaries: Boundary::defaults(),
            pattern: None,
            delim: String::new(),
        }
    }
}

impl Converter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn convert<T>(&self, s: T) -> String
    where
        T: AsRef<str>,
    {
        let words = split(&s, &self.boundaries);
        if let Some(p) = self.pattern {
            let words = words.iter().map(|s| s.as_ref()).collect::<Vec<&str>>();
            p.mutate(&words).join(&self.delim)
        } else {
            words.join(&self.delim)
        }
    }

    pub fn to_case(mut self, case: Case) -> Self {
        self.pattern = Some(case.pattern());
        self.delim = case.delim().to_string();
        self
    }

    pub fn from_case(mut self, case: Case) -> Self {
        self.boundaries = case.boundaries();
        self
    }

    pub fn set_boundaries(mut self, bs: &[Boundary]) -> Self {
        self.boundaries = bs.to_vec();
        self
    }

    pub fn add_boundary(mut self, b: Boundary) -> Self {
        self.boundaries.push(b);
        self
    }

    pub fn add_boundaries(mut self, bs: &[Boundary]) -> Self {
        self.boundaries.extend(bs);
        self
    }

    pub fn remove_boundary(mut self, b: Boundary) -> Self {
        self.boundaries.retain(|&x| x != b);
        self
    }

    pub fn remove_boundaries(mut self, bs: &[Boundary]) -> Self {
        for b in bs {
            self.boundaries.retain(|&x| x != *b);
        }
        self
    }

    pub fn set_delim<T>(mut self, d: T) -> Self
    where
        T: ToString,
    {
        self.delim = d.to_string();
        self
    }

    pub fn remove_delim(mut self) -> Self {
        self.delim = String::new();
        self
    }

    pub fn set_pattern(mut self, p: Pattern) -> Self {
        self.pattern = Some(p);
        self
    }

    pub fn remove_pattern(mut self) -> Self {
        self.pattern = None;
        self
    }
}
