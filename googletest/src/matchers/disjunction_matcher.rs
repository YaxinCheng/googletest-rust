// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::matcher::{Matcher, MatcherResult};
use std::{fmt::Debug, marker::PhantomData};

/// Matcher created by [`OrMatcherExt::or`].
///
/// **For internal use only. API stablility is not guaranteed!**
#[doc(hidden)]
pub struct DisjunctionMatcher<T: ?Sized, M1, M2> {
    m1: M1,
    m2: M2,
    phantom: PhantomData<T>,
}

impl<T: ?Sized, M1, M2> DisjunctionMatcher<T, M1, M2> {
    pub(crate) fn new(m1: M1, m2: M2) -> Self {
        Self { m1, m2, phantom: Default::default() }
    }
}

impl<T: Debug + ?Sized, M1: Matcher<ActualT = T>, M2: Matcher<ActualT = T>> Matcher
    for DisjunctionMatcher<T, M1, M2>
{
    type ActualT = T;

    fn matches(&self, actual: &T) -> MatcherResult {
        match (self.m1.matches(actual), self.m2.matches(actual)) {
            (MatcherResult::DoesNotMatch, MatcherResult::DoesNotMatch) => {
                MatcherResult::DoesNotMatch
            }
            _ => MatcherResult::Matches,
        }
    }

    fn explain_match(&self, actual: &T) -> String {
        format!("{} and\n  {}", self.m1.explain_match(actual), self.m2.explain_match(actual))
    }

    fn describe(&self, matcher_result: MatcherResult) -> String {
        format!("{}, or {}", self.m1.describe(matcher_result), self.m2.describe(matcher_result))
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use indoc::indoc;

    #[test]
    fn or_true_true_matches() -> Result<()> {
        verify_that!(1, anything().or(anything()))
    }

    #[test]
    fn or_true_false_matches() -> Result<()> {
        verify_that!(1, anything().or(not(anything())))
    }

    #[test]
    fn or_false_true_matches() -> Result<()> {
        verify_that!(1, not(anything()).or(anything()))
    }

    #[test]
    fn or_false_false_does_not_match() -> Result<()> {
        let result = verify_that!(1, not(anything()).or(not(anything())));
        verify_that!(
            result,
            err(displays_as(contains_substring(indoc!(
                "
                Value of: 1
                Expected: never matches, or never matches
                Actual: 1,
                  which is anything and
                  which is anything
                "
            ))))
        )
    }

    #[test]
    fn chained_or_matches() -> Result<()> {
        verify_that!(10, eq(1).or(eq(5)).or(ge(9)))
    }

    #[test]
    fn works_with_str_slices() -> Result<()> {
        verify_that!("A string", ends_with("A").or(ends_with("string")))
    }

    #[test]
    fn works_with_owned_strings() -> Result<()> {
        verify_that!("A string".to_string(), ends_with("A").or(ends_with("string")))
    }
}
