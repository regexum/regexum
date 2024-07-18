//! # regexum
//!
//! This crate contains handy macros and more for regex-matched enums.
//!

#![cfg_attr(not(feature = "std"), no_std)]

extern crate regexum_macros;

use core::marker::PhantomData;
use regex::{Regex, RegexSet};
pub use regex::Captures;
pub use regexum_macros::Patternize;

#[derive(Debug)]
pub enum Error {
    Unrecognized,
    MissingCapture {
        variant: String,
        pattern: String,
        name: String,
    },
    InvalidValue {
        variant: String,
        pattern: String,
        capture: String,
        inner: Box<dyn std::error::Error + 'static>,
    },
}

pub type Result<T> = core::result::Result<T, Error>;

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Unrecognized => write!(f, "unrecognized string"),
            Error::MissingCapture { variant, name, .. } => {
                write!(f, "missing capture \"{}\" in variant \"{}\"", variant, name)
            }
            Error::InvalidValue {
                  ..
            } => write!(f, "invalid value"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::InvalidValue { inner, .. } => Some(inner.as_ref()),
            _ => None,
        }
    }
}

pub trait Patternize: Sized {
    fn patterns() -> &'static [&'static str];
    fn from_captures<'c>(index: usize, captures: Captures<'c>) -> Result<Self>;
    fn matcher() -> Matcher<Self> {
        Matcher::new()
    }
}

pub struct Matcher<P: Patternize> {
    patterns: Vec<Regex>,
    set: RegexSet,
    _phantom: PhantomData<P>,
}

impl<P: Patternize> Matcher<P> {
    fn new() -> Self {
        Self {
            patterns: P::patterns().iter().map(|s| Regex::new(s).unwrap()).collect(),
            set: RegexSet::new(P::patterns()).unwrap(),
            _phantom: PhantomData,
        }
    }
    pub fn patterns(&self) -> &[Regex] {
        &self.patterns
    }
    pub fn inner(&self) -> &RegexSet {
        &self.set
    }
    pub fn into_inner(self) -> RegexSet  {
        self.set
    }
    pub fn captures<'s>(&self, str: impl AsRef<str>) -> Result<P> {
        let str = str.as_ref();
        let candidates = self.set.matches(str);
        let index = candidates.iter().next().ok_or(Error::Unrecognized)?;
        let captures = self.patterns.get(index).unwrap().captures(str).unwrap();
        P::from_captures(index, captures)
    }
}
