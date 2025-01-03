use crate::util::re;
use crate::util::re::MatchTuple;
use regex::Regex;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
pub struct LineInput<'a>(&'a str);

impl<'a> LineInput<'a> {
    pub fn new(s: &'a str) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &'a str {
        self.0
    }

    pub fn single<T>(&self) -> T
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        self.0.parse().unwrap()
    }

    pub fn chars(&self) -> Vec<char> {
        self.0.chars().collect()
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.0.bytes().collect()
    }

    pub fn digits(&self) -> Vec<usize> {
        self.0.bytes().map(|b| (b - b'0') as usize).collect()
    }

    pub fn split<T>(&self, pattern: &str) -> Vec<T>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        self.0.split(pattern).map(|s| s.parse().unwrap()).collect()
    }

    pub fn split_whitespace<T>(&self) -> Vec<T>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        self.0
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect()
    }

    pub fn split_once(&self, pattern: &str) -> (Self, Self) {
        let (a, b) = self.0.split_once(pattern).unwrap();
        (Self(a), Self(b))
    }

    pub fn parse_with_regex<T>(&self, re: &Regex) -> T
    where
        T: MatchTuple<'a>,
    {
        re::parse_with_regex(re, self.0).unwrap()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RawInput<'a>(&'a str);

impl<'a> RawInput<'a> {
    pub fn new(s: &'a str) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &'a str {
        self.0
    }

    pub fn single_line<F, T>(&self, f: F) -> T
    where
        F: Fn(LineInput) -> T,
    {
        let line = self.0.lines().next().unwrap();
        f(LineInput(line))
    }

    pub fn per_line<'b, F, T>(&self, f: F) -> impl Iterator<Item = T> + 'a
    where
        F: Fn(LineInput) -> T + 'static,
    {
        self.0.lines().map(move |line| f(LineInput(line)))
    }

    pub fn grouped_lines<F, T>(&self, f: F) -> impl Iterator<Item = Vec<T>> + 'a
    where
        F: Fn(LineInput) -> T + 'static,
    {
        self.0
            .split("\n\n")
            .map(move |group| group.lines().map(|line| f(LineInput(line))).collect())
    }

    pub fn split_once_on_empty_line(&self) -> (Self, Self) {
        let (a, b) = self.0.split_once("\n\n").unwrap();
        (Self(a), Self(b))
    }
}
