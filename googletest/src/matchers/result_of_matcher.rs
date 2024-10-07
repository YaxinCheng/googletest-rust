use crate::description::Description;
use crate::matcher::{Matcher, MatcherBase, MatcherResult};
use std::borrow::Borrow;
use std::fmt::Debug;
use std::marker::PhantomData;

pub fn result_of<
    I,
    O: Borrow<T>,
    T: Copy + Debug,
    Callable: Fn(I) -> O,
    InnerMatcher: for<'a> Matcher<&'a T>,
>(
    callable: Callable,
    inner_matcher: InnerMatcher,
) -> ResultOfMatcher<I, O, T, Callable, InnerMatcher> {
    ResultOfMatcher { callable, inner_matcher, _phantom: PhantomData }
}

#[derive(MatcherBase)]
pub struct ResultOfMatcher<
    I,
    O: Borrow<T>,
    T: Copy + Debug,
    Callable: Fn(I) -> O,
    InnerMatcher: for<'a> Matcher<&'a T>,
> {
    callable: Callable,
    inner_matcher: InnerMatcher,
    _phantom: PhantomData<(I, O, T)>,
}

impl<
        I: Copy + Debug,
        O: Borrow<T>,
        T: Debug + Copy,
        CallableT: Fn(I) -> O,
        InnerMatcherT: for<'a> Matcher<&'a T>,
    > Matcher<I> for ResultOfMatcher<I, O, T, CallableT, InnerMatcherT>
{
    fn matches(&self, actual: I) -> MatcherResult {
        let mapped = (self.callable)(actual);
        self.inner_matcher.matches(mapped.borrow())
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match matcher_result {
            MatcherResult::Match => format!(
                "is mapped by the given callable to a value that {}",
                self.inner_matcher.describe(MatcherResult::Match)
            ),
            MatcherResult::NoMatch => format!(
                "is mapped by the given callable to a value that {}",
                self.inner_matcher.describe(MatcherResult::NoMatch)
            ),
        }
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::result_of;
    use crate::matcher::MatcherResult;
    use crate::prelude::*;
    #[test]
    fn result_of_match_with_value() -> Result<()> {
        let matcher = result_of(|value| value + 1, eq(&2));
        let value = 1;
        let result = matcher.matches(value);
        verify_that!(result, eq(MatcherResult::Match))
    }

    #[test]
    fn result_of_match_with_different_value() -> Result<()> {
        let matcher = result_of(|value| value + 1, eq(&2));
        let value = 2;
        let result = matcher.matches(value);
        verify_that!(result, eq(MatcherResult::NoMatch))
    }

    #[test]
    fn result_of_match_with_reference() -> Result<()> {
        let matcher = result_of(|s: &str| s.to_uppercase(), eq("HELLO"));
        let value = "hello";
        let result = matcher.matches(value);
        verify_that!(result, eq(MatcherResult::Match))
    }
}
