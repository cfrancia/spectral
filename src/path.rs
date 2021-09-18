use super::{AssertionFailure, DescriptiveSpec, Spec};

use std::borrow::Borrow;
use std::path::{Path, PathBuf};

pub trait PathAssertions {
    fn exists(&mut self);
    fn does_not_exist(&mut self);
    fn is_a_file(&mut self);
    fn is_a_directory(&mut self);
    fn has_file_name<'r, E: Borrow<&'r str>>(&mut self, expected_file_name: E);
}

impl<'s> PathAssertions for Spec<'s, &'s Path> {
    /// Asserts that the subject `Path` refers to an existing location.
    ///
    /// ```rust, ignore
    /// assert_that(&Path::new("/tmp/file")).exists();
    /// ```
    fn exists(&mut self) {
        exists(self.subject, self)
    }

    /// Asserts that the subject `Path` does not refer to an existing location.
    ///
    /// ```rust
    /// # use spectral::prelude::*;
    /// # use std::path::Path;
    /// assert_that(&Path::new("/tmp/file")).does_not_exist();
    /// ```
    fn does_not_exist(&mut self) {
        does_not_exist(self.subject, self)
    }

    /// Asserts that the subject `Path` refers to an existing file.
    ///
    /// ```rust, ignore
    /// assert_that(&Path::new("/tmp/file")).is_a_file();
    /// ```
    fn is_a_file(&mut self) {
        is_a_file(self.subject, self)
    }

    /// Asserts that the subject `Path` refers to an existing directory.
    ///
    /// ```rust, ignore
    /// assert_that(&Path::new("/tmp/dir/")).is_a_directory();
    /// ```
    fn is_a_directory(&mut self) {
        is_a_directory(self.subject, self)
    }

    /// Asserts that the subject `Path` has the expected file name.
    ///
    /// ```rust
    /// # use spectral::prelude::*;
    /// # use std::path::Path;
    /// assert_that(&Path::new("/tmp/file")).has_file_name(&"file");
    /// ```
    fn has_file_name<'r, E: Borrow<&'r str>>(&mut self, expected_file_name: E) {
        has_file_name(self.subject, expected_file_name.borrow(), self)
    }
}

impl<'s> PathAssertions for Spec<'s, PathBuf> {
    /// Asserts that the subject `PathBuf` refers to an existing location.
    ///
    /// ```rust, ignore
    /// assert_that(&PathBuf::from("/tmp/file")).exists();
    /// ```
    fn exists(&mut self) {
        exists(self.subject.as_path(), self)
    }

    /// Asserts that the subject `PathBuf` does not refer to an existing location.
    ///
    /// ```rust, ignore
    /// assert_that(&PathBuf::from("/tmp/file")).does_not_exist();
    /// ```
    fn does_not_exist(&mut self) {
        does_not_exist(self.subject.as_path(), self)
    }

    /// Asserts that the subject `PathBuf` refers to an existing file.
    ///
    /// ```rust, ignore
    /// assert_that(&PathBuf::from("/tmp/file")).is_a_file();
    /// ```
    fn is_a_file(&mut self) {
        is_a_file(self.subject.as_path(), self)
    }

    /// Asserts that the subject `PathBuf` refers to an existing directory.
    ///
    /// ```rust, ignore
    /// assert_that(&PathBuf::from("/tmp/dir/")).is_a_directory();
    /// ```
    fn is_a_directory(&mut self) {
        is_a_directory(self.subject.as_path(), self)
    }

    /// Asserts that the subject `PathBuf` has the expected file name.
    /// ```rust, ignore
    /// assert_that(&PathBuf::from("/tmp/file")).has_file_name(&"file");
    /// ```
    fn has_file_name<'r, E: Borrow<&'r str>>(&mut self, expected_file_name: E) {
        has_file_name(self.subject.as_path(), expected_file_name.borrow(), self)
    }
}

fn exists<'s, S: DescriptiveSpec<'s>>(subject: &Path, spec: &'s S) {
    if !subject.exists() {
        AssertionFailure::from_spec(spec)
            .with_expected(format!("Path of <{:?}> to exist", subject))
            .with_actual(format!("a non-existent Path"))
            .fail();
    }
}

fn does_not_exist<'s, S: DescriptiveSpec<'s>>(subject: &Path, spec: &'s S) {
    if subject.exists() {
        AssertionFailure::from_spec(spec)
            .with_expected(format!("Path of <{:?}> to not exist", subject))
            .with_actual(format!("a resolvable Path"))
            .fail();
    }
}

fn is_a_file<'s, S: DescriptiveSpec<'s>>(subject: &Path, spec: &'s S) {
    if !subject.is_file() {
        AssertionFailure::from_spec(spec)
            .with_expected(format!("Path of <{:?}> to be a file", subject))
            .with_actual(format!("not a resolvable file"))
            .fail();
    }
}

fn is_a_directory<'s, S: DescriptiveSpec<'s>>(subject: &Path, spec: &'s S) {
    if !subject.is_dir() {
        AssertionFailure::from_spec(spec)
            .with_expected(format!("Path of <{:?}> to be a directory", subject))
            .with_actual(format!("not a resolvable directory"))
            .fail();
    }
}

fn has_file_name<'s, S: DescriptiveSpec<'s>>(
    subject: &Path,
    expected_file_name: &str,
    spec: &'s S,
) {
    let subject_file_name = match subject.file_name() {
        Some(os_string) => match os_string.to_str() {
            Some(val) => val,
            None => {
                fail_from_file_name(
                    spec,
                    expected_file_name,
                    format!("an invalid UTF-8 file name"),
                );
                unreachable!();
            }
        },
        None => {
            fail_from_file_name(
                spec,
                expected_file_name,
                format!("a non-resolvable path <{:?}>", subject),
            );
            unreachable!();
        }
    };

    if !subject_file_name.eq(expected_file_name) {
        fail_from_file_name(spec, expected_file_name, format!("<{}>", subject_file_name));
    }
}

fn fail_from_file_name<'s, S: DescriptiveSpec<'s>>(spec: &'s S, expected: &str, actual: String) {
    AssertionFailure::from_spec(spec)
        .with_expected(build_file_name_message(expected))
        .with_actual(actual)
        .fail();
}

fn build_file_name_message(file_name: &str) -> String {
    format!("Path with file name of <{}>", file_name)
}

#[cfg(test)]
mod tests {

    use super::super::prelude::*;

    use std::path::{Path, PathBuf};

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
    pub fn should_not_panic_if_path_does_not_exist_when_expected() {
        let failing_path = MANIFEST_PATH.to_string() + "/does-not-exist";
        assert_that(&Path::new(&failing_path)).does_not_exist();
    }

    #[test]
    // It's unfortunately a bit hard to expect a message without knowing the manifest path
    #[should_panic]
    pub fn should_panic_if_path_exists_when_not_expected() {
        assert_that(&Path::new(MANIFEST_PATH)).does_not_exist();
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
    pub fn has_file_name_should_allow_multiple_borrow_forms_for_path() {
        let path = MANIFEST_PATH.to_string() + "/Cargo.toml";
        assert_that(&Path::new(&path)).has_file_name("Cargo.toml");
        assert_that(&Path::new(&path)).has_file_name(&mut "Cargo.toml");
        assert_that(&Path::new(&path)).has_file_name(&"Cargo.toml");
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

    #[test]
    pub fn should_not_panic_if_pathbuf_exists() {
        assert_that(&PathBuf::from(MANIFEST_PATH)).exists();
    }

    #[test]
    // It's unfortunately a bit hard to expect a message without knowing the manifest path
    #[should_panic]
    pub fn should_panic_if_pathbuf_does_not_exist() {
        let failing_path = MANIFEST_PATH.to_string() + "/does-not-exist";
        assert_that(&PathBuf::from(&failing_path)).exists();
    }

    #[test]
    pub fn should_not_panic_if_pathbuf_represents_a_directory() {
        assert_that(&PathBuf::from(MANIFEST_PATH)).is_a_directory();
    }

    #[test]
    pub fn should_not_panic_if_pathbuf_does_not_exist_when_expected() {
        let failing_path = MANIFEST_PATH.to_string() + "/does-not-exist";
        assert_that(&PathBuf::from(&failing_path)).does_not_exist();
    }

    #[test]
    // It's unfortunately a bit hard to expect a message without knowing the manifest path
    #[should_panic]
    pub fn should_panic_if_pathbuf_exists_when_not_expected() {
        assert_that(&PathBuf::from(MANIFEST_PATH)).does_not_exist();
    }

    #[test]
    // It's unfortunately a bit hard to expect a message without knowing the manifest path
    #[should_panic]
    pub fn should_panic_if_pathbuf_does_not_represent_a_directory() {
        let path = MANIFEST_PATH.to_string() + "/Cargo.toml";
        assert_that(&PathBuf::from(&path)).is_a_directory();
    }

    #[test]
    pub fn should_not_panic_if_pathbuf_represents_a_file() {
        let path = MANIFEST_PATH.to_string() + "/Cargo.toml";
        assert_that(&PathBuf::from(&path)).is_a_file();
    }

    #[test]
    // It's unfortunately a bit hard to expect a message without knowing the manifest path
    #[should_panic]
    pub fn should_panic_if_pathbuf_does_not_represent_a_file() {
        assert_that(&PathBuf::from(&MANIFEST_PATH)).is_a_file();
    }

    #[test]
    pub fn has_file_name_should_allow_multiple_borrow_forms_for_pathbuf() {
        let path = MANIFEST_PATH.to_string() + "/Cargo.toml";
        assert_that(&PathBuf::from(&path)).has_file_name("Cargo.toml");
        assert_that(&PathBuf::from(&path)).has_file_name(&mut "Cargo.toml");
        assert_that(&PathBuf::from(&path)).has_file_name(&"Cargo.toml");
    }

    #[test]
    pub fn should_not_panic_if_pathbuf_has_correct_file_name() {
        let path = MANIFEST_PATH.to_string() + "/Cargo.toml";
        assert_that(&PathBuf::from(&path)).has_file_name(&"Cargo.toml");
    }

    #[test]
    // It's unfortunately a bit hard to expect a message without knowing the manifest path
    #[should_panic]
    pub fn should_panic_if_pathbuf_does_not_have_correct_file_name() {
        let path = MANIFEST_PATH.to_string() + "/Cargo.toml";
        assert_that(&PathBuf::from(&path)).has_file_name(&"pom.xml");
    }

    #[test]
    // It's unfortunately a bit hard to expect a message without knowing the manifest path
    #[should_panic]
    pub fn should_panic_if_pathbuf_does_not_have_a_file_name() {
        let path = MANIFEST_PATH.to_string() + "/..";
        assert_that(&PathBuf::from(&path)).has_file_name(&"pom.xml");
    }
}
