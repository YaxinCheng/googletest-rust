// Copyright 2023 Google LLC
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

#[cfg(not(google3))]
use googletest::elements_are;
use googletest::matcher::Matcher;
#[cfg(not(google3))]
use googletest::matchers;
use googletest::{google_test, verify_that, Result};
use indoc::indoc;
#[cfg(google3)]
use matchers::elements_are;
use matchers::{contains_substring, displays_as, eq, err, not};

#[google_test]
fn elements_are_matches_vector() -> Result<()> {
    let value = vec![1, 2, 3];
    verify_that!(value, elements_are![eq(1), eq(2), eq(3)])
}

#[google_test]
fn elements_are_matches_slice() -> Result<()> {
    let value = vec![1, 2, 3];
    let slice = value.as_slice();
    verify_that!(*slice, elements_are![eq(1), eq(2), eq(3)])
}

#[google_test]
fn elements_are_matches_array() -> Result<()> {
    verify_that!([1, 2, 3], elements_are![eq(1), eq(2), eq(3)])
}

#[google_test]
fn elements_are_supports_trailing_comma() -> Result<()> {
    let value = vec![1, 2, 3];
    verify_that!(value, elements_are![eq(1), eq(2), eq(3),])
}

#[google_test]
fn elements_are_returns_no_match_when_expected_and_actual_sizes_differ() -> Result<()> {
    let value = vec![1, 2];
    verify_that!(value, not(elements_are![eq(1), eq(2), eq(3)]))
}

#[google_test]
fn elements_are_produces_correct_failure_message() -> Result<()> {
    let result = verify_that!(vec![1, 4, 3], elements_are![eq(1), eq(2), eq(3)]);
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            "
                Value of: vec![1, 4, 3]
                Expected: has elements:
                  0. is equal to 1
                  1. is equal to 2
                  2. is equal to 3
                Actual: [
                    1,
                    4,
                    3,
                ], where element #1 is 4, which isn't equal to 2"
        ))))
    )
}

#[google_test]
fn elements_are_produces_correct_failure_message_nested() -> Result<()> {
    let result = verify_that!(
        vec![vec![0, 1], vec![1, 2]],
        elements_are![elements_are![eq(1), eq(2)], elements_are![eq(2), eq(3)]]
    );
    verify_that!(
        result,
        err(displays_as(contains_substring(indoc!(
            "
                Value of: vec![vec! [0, 1], vec! [1, 2]]
                Expected: has elements:
                  0. has elements:
                       0. is equal to 1
                       1. is equal to 2
                  1. has elements:
                       0. is equal to 2
                       1. is equal to 3
                Actual: [
                    [
                        0,
                        1,
                    ],
                    [
                        1,
                        2,
                    ],
                ], where:
                  * element #0 is [0, 1], where:
                      * element #0 is 0, which isn't equal to 1
                      * element #1 is 1, which isn't equal to 2
                  * element #1 is [1, 2], where:
                      * element #0 is 1, which isn't equal to 2
                      * element #1 is 2, which isn't equal to 3"
        ))))
    )
}

#[google_test]
fn elements_are_explain_match_wrong_size() -> Result<()> {
    verify_that!(
        elements_are![eq(1)].explain_match(&vec![1, 2]),
        displays_as(eq("whose size is 2"))
    )
}