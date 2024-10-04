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

fn main() {}

// Mixing rstest and gtest should not result in any compiler
// warnings. The fact that this successfully compiles is part of the test.
#[deny(warnings)]
#[cfg(test)]
mod tests {
    use googletest::prelude::*;
    use rstest::rstest;

    // The following tests use `#[googletest::test]` instead of `#[gtest]` because
    // `#[rstest]` requires a third-party testing attribute to be called
    // literally `test`.
    #[rstest]
    #[googletest::test]
    fn test_should_work_with_rstest_first() -> Result<()> {
        verify_that!(1, eq(1))
    }

    #[rstest::rstest]
    #[googletest::test]
    fn test_should_work_with_qualified_rstest_first() -> Result<()> {
        verify_that!(1, eq(1))
    }

    #[gtest]
    #[rstest]
    fn test_should_work_with_rstest_second() -> Result<()> {
        verify_that!(1, eq(1))
    }

    #[gtest]
    #[rstest::rstest]
    fn test_should_work_with_qualified_rstest_second() -> Result<()> {
        verify_that!(1, eq(1))
    }

    #[rstest]
    #[case(1)]
    #[gtest]
    fn paramterised_test_should_work_with_rstest_first(#[case] value: u32) -> Result<()> {
        verify_that!(value, eq(value))
    }

    #[gtest]
    #[rstest]
    #[case(1)]
    fn paramterised_test_should_work_with_rstest_second(#[case] value: u32) -> Result<()> {
        verify_that!(value, eq(value))
    }

    mod submodule {
        pub use rstest::rstest as test;
    }

    #[gtest]
    #[submodule::test]
    fn test_should_work_with_qualified_test_annotation() -> Result<()> {
        verify_that!(1, eq(1))
    }

    #[gtest]
    #[test]
    fn test_should_work_with_second_test_annotation() -> Result<()> {
        verify_that!(1, eq(1))
    }

    #[rstest]
    #[googletest::test]
    fn test_should_work_with_rstest_no_return_first() {
        expect_that!(1, eq(1));
    }

    #[googletest::test]
    #[rstest]
    fn test_should_work_with_rstest_no_return_second() {
        expect_that!(1, eq(1));
    }
}
