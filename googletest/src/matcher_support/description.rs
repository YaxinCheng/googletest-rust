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

use std::fmt::{Display, Formatter, Result};

/// Helper structure to build better output of
/// [`Matcher::describe`][crate::matcher::Matcher::describe] and
/// [`Matcher::explain_match`][crate::matcher::Matcher::explain_match]. This
/// is especially useful with composed matchers and matchers over containers.
///
/// It provides simple operations to lazily format lists of strings.
///
/// Usage:
/// ```ignore
/// let iter: impl Iterator<String> = ...
/// format!("{}", iter.collect::<Description>().indent().bullet_list())
/// ```
///
/// To construct a [`Description`], use `Iterator<Item=String>::collect()`.
/// Each element of the collected iterator will be separated by a
/// newline when displayed. The elements may be multi-line, but they will
/// nevertheless be indented consistently.
///
/// Note that a newline will only be added between each element, but not
/// after the last element. This makes it simpler to keep
/// [`Matcher::describe`][crate::matcher::Matcher::describe]
/// and [`Matcher::explain_match`][crate::matcher::Matcher::explain_match]
/// consistent with simpler [`Matchers`][crate::matcher::Matcher].
///
/// They can also be indented, enumerated and or
/// bullet listed if [`Description::indent`], [`Description::enumerate`], or
/// respectively [`Description::bullet_list`] has been called.
#[derive(Debug)]
pub struct Description {
    elements: Vec<String>,
    indent_mode: IndentMode,
    list_style: ListStyle,
}

#[derive(Debug)]
enum IndentMode {
    NoIndent,
    EveryLine,
    AllExceptFirstLine,
}

#[derive(Debug)]
enum ListStyle {
    NoList,
    Bullet,
    Enumerate,
}

struct IndentationSizes {
    first_line_indent: usize,
    first_line_of_element_indent: usize,
    enumeration_padding: usize,
    other_line_indent: usize,
}

/// Number of space used to indent lines when no alignement is required.
const INDENTATION_SIZE: usize = 2;

impl Description {
    /// Indents the lines in elements of this description.
    ///
    /// This operation will be performed lazily when [`self`] is displayed.
    ///
    /// This will indent every line inside each element.
    ///
    /// For example:
    ///
    /// ```
    /// # use googletest::prelude::*;
    /// # use googletest::matcher_support::description::Description;
    /// let description = std::iter::once("A B C\nD E F".to_string()).collect::<Description>();
    /// verify_that!(description.indent(), displays_as(eq("  A B C\n  D E F")))
    /// # .unwrap();
    /// ```
    pub fn indent(self) -> Self {
        Self { indent_mode: IndentMode::EveryLine, ..self }
    }

    /// Indents the lines in elements of this description except for the first
    /// line.
    ///
    /// This is similar to [`Self::indent`] except that the first line is not
    /// indented. This is useful when the first line has already been indented
    /// in the output.
    ///
    /// For example:
    ///
    /// ```
    /// # use googletest::prelude::*;
    /// # use googletest::matcher_support::description::Description;
    /// let description = std::iter::once("A B C\nD E F".to_string()).collect::<Description>();
    /// verify_that!(description.indent_except_first_line(), displays_as(eq("A B C\n  D E F")))
    /// # .unwrap();
    /// ```
    pub fn indent_except_first_line(self) -> Self {
        Self { indent_mode: IndentMode::AllExceptFirstLine, ..self }
    }

    /// Bullet lists the elements of [`self`].
    ///
    /// This operation will be performed lazily when [`self`] is displayed.
    ///
    /// Note that this will only bullet list each element, not each line
    /// in each element.
    ///
    /// For instance:
    ///
    /// ```
    /// # use googletest::prelude::*;
    /// # use googletest::matcher_support::description::Description;
    /// let description = std::iter::once("A B C\nD E F".to_string()).collect::<Description>();
    /// verify_that!(description.bullet_list(), displays_as(eq("* A B C\n  D E F")))
    /// # .unwrap();
    /// ```
    pub fn bullet_list(self) -> Self {
        Self { list_style: ListStyle::Bullet, ..self }
    }

    /// Enumerates the elements of [`self`].
    ///
    /// This operation will be performed lazily when [`self`] is displayed.
    ///
    /// Note that this will only enumerate each element, not each line in
    /// each element.
    ///
    /// For instance:
    ///
    /// ```
    /// # use googletest::prelude::*;
    /// # use googletest::matcher_support::description::Description;
    /// let description = std::iter::once("A B C\nD E F".to_string()).collect::<Description>();
    /// verify_that!(description.enumerate(), displays_as(eq("0. A B C\n   D E F")))
    /// # .unwrap();
    /// ```
    pub fn enumerate(self) -> Self {
        Self { list_style: ListStyle::Enumerate, ..self }
    }

    /// Returns the length of elements.
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Returns whether the set of elements is empty.
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    fn indentation_sizes(&self) -> IndentationSizes {
        let first_line_indent =
            if matches!(self.indent_mode, IndentMode::EveryLine) { INDENTATION_SIZE } else { 0 };
        let first_line_of_element_indent =
            if !matches!(self.indent_mode, IndentMode::NoIndent) { INDENTATION_SIZE } else { 0 };
        // Number of digit of the last index. For instance, an array of length 13 will
        // have 12 as last index (we start at 0), which have a digit size of 2.
        let enumeration_padding = if self.elements.len() > 1 {
            ((self.elements.len() - 1) as f64).log10().floor() as usize + 1
        } else {
            // Avoid negative logarithm when there is only 0 or 1 element.
            1
        };

        let other_line_indent = first_line_of_element_indent
            + match self.list_style {
                ListStyle::NoList => 0,
                ListStyle::Bullet => "* ".len(),
                ListStyle::Enumerate => enumeration_padding + ". ".len(),
            };
        IndentationSizes {
            first_line_indent,
            first_line_of_element_indent,
            enumeration_padding,
            other_line_indent,
        }
    }
}

impl Display for Description {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let IndentationSizes {
            mut first_line_indent,
            first_line_of_element_indent,
            enumeration_padding,
            other_line_indent,
        } = self.indentation_sizes();

        let mut first = true;
        for (idx, element) in self.elements.iter().enumerate() {
            let mut lines = element.lines();
            if let Some(line) = lines.next() {
                if first {
                    first = false;
                } else {
                    writeln!(f)?;
                }
                match self.list_style {
                    ListStyle::NoList => {
                        write!(f, "{:first_line_indent$}{line}", "")?;
                    }
                    ListStyle::Bullet => {
                        write!(f, "{:first_line_indent$}* {line}", "")?;
                    }
                    ListStyle::Enumerate => {
                        write!(
                            f,
                            "{:first_line_indent$}{:>enumeration_padding$}. {line}",
                            "", idx,
                        )?;
                    }
                }
            }
            for line in lines {
                writeln!(f)?;
                write!(f, "{:other_line_indent$}{line}", "")?;
            }
            first_line_indent = first_line_of_element_indent;
        }
        Ok(())
    }
}

impl FromIterator<String> for Description {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = String>,
    {
        Self {
            elements: iter.into_iter().collect(),
            indent_mode: IndentMode::NoIndent,
            list_style: ListStyle::NoList,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Description;
    use crate::prelude::*;
    use indoc::indoc;

    #[test]
    fn description_single_element() -> Result<()> {
        let description = ["A B C".to_string()].into_iter().collect::<Description>();
        verify_that!(description, displays_as(eq("A B C")))
    }

    #[test]
    fn description_two_elements() -> Result<()> {
        let description =
            ["A B C".to_string(), "D E F".to_string()].into_iter().collect::<Description>();
        verify_that!(description, displays_as(eq("A B C\nD E F")))
    }

    #[test]
    fn description_indent_single_element() -> Result<()> {
        let description = ["A B C".to_string()].into_iter().collect::<Description>().indent();
        verify_that!(description, displays_as(eq("  A B C")))
    }

    #[test]
    fn description_indent_two_elements() -> Result<()> {
        let description = ["A B C".to_string(), "D E F".to_string()]
            .into_iter()
            .collect::<Description>()
            .indent();
        verify_that!(description, displays_as(eq("  A B C\n  D E F")))
    }

    #[test]
    fn description_indent_two_elements_except_first_line() -> Result<()> {
        let description = ["A B C".to_string(), "D E F".to_string()]
            .into_iter()
            .collect::<Description>()
            .indent_except_first_line();
        verify_that!(description, displays_as(eq("A B C\n  D E F")))
    }

    #[test]
    fn description_indent_single_element_two_lines() -> Result<()> {
        let description =
            ["A B C\nD E F".to_string()].into_iter().collect::<Description>().indent();
        verify_that!(description, displays_as(eq("  A B C\n  D E F")))
    }

    #[test]
    fn description_indent_single_element_two_lines_except_first_line() -> Result<()> {
        let description = ["A B C\nD E F".to_string()]
            .into_iter()
            .collect::<Description>()
            .indent_except_first_line();
        verify_that!(description, displays_as(eq("A B C\n  D E F")))
    }

    #[test]
    fn description_bullet_single_element() -> Result<()> {
        let description = ["A B C".to_string()].into_iter().collect::<Description>().bullet_list();
        verify_that!(description, displays_as(eq("* A B C")))
    }

    #[test]
    fn description_bullet_two_elements() -> Result<()> {
        let description = ["A B C".to_string(), "D E F".to_string()]
            .into_iter()
            .collect::<Description>()
            .bullet_list();
        verify_that!(description, displays_as(eq("* A B C\n* D E F")))
    }

    #[test]
    fn description_bullet_single_element_two_lines() -> Result<()> {
        let description =
            ["A B C\nD E F".to_string()].into_iter().collect::<Description>().bullet_list();
        verify_that!(description, displays_as(eq("* A B C\n  D E F")))
    }

    #[test]
    fn description_bullet_single_element_two_lines_indent_except_first_line() -> Result<()> {
        let description = ["A B C\nD E F".to_string()]
            .into_iter()
            .collect::<Description>()
            .bullet_list()
            .indent_except_first_line();
        verify_that!(description, displays_as(eq("* A B C\n    D E F")))
    }

    #[test]
    fn description_bullet_two_elements_indent_except_first_line() -> Result<()> {
        let description = ["A B C".to_string(), "D E F".to_string()]
            .into_iter()
            .collect::<Description>()
            .bullet_list()
            .indent_except_first_line();
        verify_that!(description, displays_as(eq("* A B C\n  * D E F")))
    }

    #[test]
    fn description_enumerate_single_element() -> Result<()> {
        let description = ["A B C".to_string()].into_iter().collect::<Description>().enumerate();
        verify_that!(description, displays_as(eq("0. A B C")))
    }

    #[test]
    fn description_enumerate_two_elements() -> Result<()> {
        let description = ["A B C".to_string(), "D E F".to_string()]
            .into_iter()
            .collect::<Description>()
            .enumerate();
        verify_that!(description, displays_as(eq("0. A B C\n1. D E F")))
    }

    #[test]
    fn description_enumerate_single_element_two_lines() -> Result<()> {
        let description =
            ["A B C\nD E F".to_string()].into_iter().collect::<Description>().enumerate();
        verify_that!(description, displays_as(eq("0. A B C\n   D E F")))
    }

    #[test]
    fn description_enumerate_correct_indentation_with_large_index() -> Result<()> {
        let description = ["A B C\nD E F"; 11]
            .into_iter()
            .map(str::to_string)
            .collect::<Description>()
            .enumerate();
        verify_that!(
            description,
            displays_as(eq(indoc!(
                "
                 0. A B C
                    D E F
                 1. A B C
                    D E F
                 2. A B C
                    D E F
                 3. A B C
                    D E F
                 4. A B C
                    D E F
                 5. A B C
                    D E F
                 6. A B C
                    D E F
                 7. A B C
                    D E F
                 8. A B C
                    D E F
                 9. A B C
                    D E F
                10. A B C
                    D E F"
            )))
        )
    }
}
