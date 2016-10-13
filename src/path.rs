use super::{AssertionFailure, Spec};

use std::path::Path;

pub trait PathSpec {
    fn exists(&mut self) -> &mut Self;
    fn is_a_file(&mut self) -> &mut Self;
    fn is_a_directory(&mut self) -> &mut Self;
    fn has_file_name(&mut self, expected_file_name: &str) -> &mut Self;
}

impl<'s> PathSpec for Spec<'s, &'s Path> {
    /// Asserts that the subject `Path` refers to an existing location.
    ///
    /// ```rust,ignore
    /// assert_that(&Path::new("/tmp/file").exists();
    /// ```
    fn exists(&mut self) -> &mut Self {
        let subject = self.subject;
        if !self.subject.exists() {
            AssertionFailure::from_spec(self)
                .with_expected(format!("Path of <{:?}> to exist", subject))
                .with_actual(format!("a non-existent Path"))
                .fail();
        }

        self
    }

    /// Asserts that the subject `Path` refers to an existing file.
    ///
    /// ```rust,ignore
    /// assert_that(&Path::new("/tmp/file").is_a_file();
    /// ```
    fn is_a_file(&mut self) -> &mut Self {
        let subject = self.subject;
        if !self.subject.is_file() {
            AssertionFailure::from_spec(self)
                .with_expected(format!("Path of <{:?}> to be a file", subject))
                .with_actual(format!("not a resolvable file"))
                .fail();
        }

        self
    }

    /// Asserts that the subject `Path` refers to an existing directory.
    ///
    /// ```rust,ignore
    /// assert_that(&Path::new("/tmp/dir/").is_a_directory();
    /// ```
    fn is_a_directory(&mut self) -> &mut Self {
        let subject = self.subject;
        if !self.subject.is_dir() {
            AssertionFailure::from_spec(self)
                .with_expected(format!("Path of <{:?}> to be a directory", subject))
                .with_actual(format!("not a resolvable directory"))
                .fail();
        }

        self
    }

    /// Asserts that the subject `Path` has the expected file name.
    ///
    /// ```rust,ignore
    /// assert_that(&Path::new("/tmp/file").has_file_name(&"file");
    /// ```
    fn has_file_name(&mut self, expected_file_name: &str) -> &mut Self {
        let subject = self.subject;

        let subject_file_name = match subject.file_name() {
            Some(os_string) => {
                match os_string.to_str() {
                    Some(val) => val,
                    None => {
                        AssertionFailure::from_spec(self)
                            .with_expected(build_file_name_message(expected_file_name))
                            .with_actual(format!("an invalid UTF-8 file name"))
                            .fail();

                        unreachable!();
                    }
                }
            }
            None => {
                AssertionFailure::from_spec(self)
                    .with_expected(build_file_name_message(expected_file_name))
                    .with_actual(format!("a non-resolvable path <{:?}>", subject))
                    .fail();

                unreachable!();
            }
        };

        if !subject_file_name.eq(expected_file_name) {
            AssertionFailure::from_spec(self)
                .with_expected(build_file_name_message(expected_file_name))
                .with_actual(format!("<{}>", subject_file_name))
                .fail();
        }

        self
    }
}

fn build_file_name_message(file_name: &str) -> String {
    format!("Path with file name of <{}>", file_name)
}

#[cfg(test)]
mod tests {

    use super::super::prelude::*;

    use std::path::Path;

    static MANIFEST_PATH: &'static str = env!("CARGO_MANIFEST_DIR");

    #[test]
    pub fn should_not_panic_if_path_exists() {
        assert_that(&Path::new(MANIFEST_PATH)).exists();
    }

    #[test]
    // It's unfortunately a bit hard to expect a message without knowing the manifest path
    #[should_panic]
    pub fn should_panic_if_path_does_not_exist() {
        let failing_path = MANIFEST_PATH.to_string() + "/does-not-exist";
        assert_that(&Path::new(&failing_path)).exists();
    }

    #[test]
    pub fn should_not_panic_if_path_represents_a_directory() {
        assert_that(&Path::new(MANIFEST_PATH)).is_a_directory();
    }

    #[test]
    // It's unfortunately a bit hard to expect a message without knowing the manifest path
    #[should_panic]
    pub fn should_panic_if_path_does_not_represent_a_directory() {
        let path = MANIFEST_PATH.to_string() + "/Cargo.toml";
        assert_that(&Path::new(&path)).is_a_directory();
    }

    #[test]
    pub fn should_not_panic_if_path_represents_a_file() {
        let path = MANIFEST_PATH.to_string() + "/Cargo.toml";
        assert_that(&Path::new(&path)).is_a_file();
    }

    #[test]
    // It's unfortunately a bit hard to expect a message without knowing the manifest path
    #[should_panic]
    pub fn should_panic_if_path_does_not_represent_a_file() {
        assert_that(&Path::new(&MANIFEST_PATH)).is_a_file();
    }

    #[test]
    pub fn should_not_panic_if_path_has_correct_file_name() {
        let path = MANIFEST_PATH.to_string() + "/Cargo.toml";
        assert_that(&Path::new(&path)).has_file_name(&"Cargo.toml");
    }

    #[test]
    // It's unfortunately a bit hard to expect a message without knowing the manifest path
    #[should_panic]
    pub fn should_panic_if_path_does_not_have_correct_file_name() {
        let path = MANIFEST_PATH.to_string() + "/Cargo.toml";
        assert_that(&Path::new(&path)).has_file_name(&"pom.xml");
    }

    #[test]
    // It's unfortunately a bit hard to expect a message without knowing the manifest path
    #[should_panic]
    pub fn should_panic_if_path_does_not_have_a_file_name() {
        let path = MANIFEST_PATH.to_string() + "/..";
        assert_that(&Path::new(&path)).has_file_name(&"pom.xml");
    }
}
