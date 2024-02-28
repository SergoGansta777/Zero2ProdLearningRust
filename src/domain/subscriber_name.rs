use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(s: String) -> Result<SubscriberName, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));

        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            Err(format!("{} is not a valid subscriber name.", s))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use claims::{assert_err, assert_ok};

    use crate::domain::SubscriberName;

    #[test]
    fn name_in_range_of_3_to_30_grapheme() {
        let name = "a".repeat(20);
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn empty_name_is_rejected() {
        let empty_name = "".to_string();
        assert_err!(SubscriberName::parse(empty_name));
        let only_whitespace = " ".to_string();
        assert_err!(SubscriberName::parse(only_whitespace));
    }

    #[test]
    fn names_containing_an_invalid_character_are_rejected() {
        let name = "a".repeat(3);
        for invalid_char in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let mut name = name.clone();
            name.push_str(&invalid_char.to_string());
            assert_err!(SubscriberName::parse(name));
        }
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "Ursula Le Guin".to_string();
        assert_ok!(SubscriberName::parse(name));
    }
}
