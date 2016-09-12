use super::Spec;

pub trait StrSpec {
    fn starts_with(self, expected: &str);
    fn ends_with(self, expected: &str);
    fn contains(self, expected: &str);
}

impl<'s> StrSpec for Spec<'s, &'s str> {
    fn starts_with(self, expected: &str) {
        let subject = self.subject;

        if !subject.starts_with(expected) {
            self.with_expected(format!("string starting with <{:?}>", expected))
                .with_actual(format!("<{:?}>", subject))
                .fail();
        }
    }

    fn ends_with(self, expected: &str) {
        let subject = self.subject;

        if !subject.ends_with(expected) {
            self.with_expected(format!("string ending with <{:?}>", expected))
                .with_actual(format!("<{:?}>", subject))
                .fail();
        }
    }

    fn contains(self, expected: &str) {
        let subject = self.subject;

        if !subject.contains(expected) {
            self.with_expected(format!("string containing <{:?}>", expected))
                .with_actual(format!("<{:?}>", subject))
                .fail();
        }
    }
}
