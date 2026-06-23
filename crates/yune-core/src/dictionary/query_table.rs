use std::collections::{btree_map, BTreeMap};

use crate::Candidate;

pub(crate) trait TableLookup {
    type PrefixCandidates<'a>: Iterator<Item = (&'a str, &'a [Candidate])>
    where
        Self: 'a;
    type AllCodes<'a>: Iterator<Item = &'a str>
    where
        Self: 'a;

    fn has_code(&self, code: &str) -> bool;

    fn exact_candidates(&self, code: &str) -> Option<&[Candidate]>;

    fn prefix_candidates<'a>(&'a self, prefix: &'a str) -> Self::PrefixCandidates<'a>;

    #[allow(dead_code)]
    fn all_codes(&self) -> Self::AllCodes<'_>;
}

pub(crate) struct HeapPrefixCandidates<'a> {
    prefix: &'a str,
    inner: btree_map::Range<'a, String, Vec<Candidate>>,
}

impl<'a> Iterator for HeapPrefixCandidates<'a> {
    type Item = (&'a str, &'a [Candidate]);

    fn next(&mut self) -> Option<Self::Item> {
        let (code, candidates) = self.inner.next()?;
        code.starts_with(self.prefix)
            .then_some((code.as_str(), candidates.as_slice()))
    }
}

pub(crate) struct HeapAllCodes<'a> {
    inner: btree_map::Keys<'a, String, Vec<Candidate>>,
}

impl<'a> Iterator for HeapAllCodes<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(String::as_str)
    }
}

impl TableLookup for BTreeMap<String, Vec<Candidate>> {
    type PrefixCandidates<'a> = HeapPrefixCandidates<'a>;
    type AllCodes<'a> = HeapAllCodes<'a>;

    fn has_code(&self, code: &str) -> bool {
        self.contains_key(code)
    }

    fn exact_candidates(&self, code: &str) -> Option<&[Candidate]> {
        self.get(code).map(Vec::as_slice)
    }

    fn prefix_candidates<'a>(&'a self, prefix: &'a str) -> Self::PrefixCandidates<'a> {
        HeapPrefixCandidates {
            prefix,
            inner: self.range(prefix.to_owned()..),
        }
    }

    fn all_codes(&self) -> Self::AllCodes<'_> {
        HeapAllCodes { inner: self.keys() }
    }
}
