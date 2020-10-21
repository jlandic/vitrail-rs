pub trait Modifier {
    fn apply(&self, source: &str) -> String;
}

pub struct CapitalizeModifier {}
impl Modifier for CapitalizeModifier {
    fn apply(&self, source: &str) -> String {
        let mut chars = source.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }
}

pub struct PluralizeModifier {}
impl PluralizeModifier {
    fn is_vowel(c: char) -> bool {
        vec!['a', 'e', 'i', 'o', 'u'].contains(&c.to_ascii_lowercase())
    }
}

impl Modifier for PluralizeModifier {
    fn apply(&self, source: &str) -> String {
        match source.chars().last() {
            None => String::new(),
            Some('s') | Some('h') | Some('x') => format!("{}es", source),
            Some('y') => {
                let root: String = source.chars().take(source.len() - 1).collect();
                if !PluralizeModifier::is_vowel(root.chars().last().unwrap()) {
                    format!("{}ies", root)
                } else {
                    format!("{}s", source)
                }
            }
            Some(_) => format!("{}s", source),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capitalize_empty_string() {
        let modifier = CapitalizeModifier {};
        assert_eq!(modifier.apply(""), "");
    }

    #[test]
    fn capitalize_valid_string() {
        let modifier = CapitalizeModifier {};
        assert_eq!(
            modifier.apply("capitalize me please"),
            "Capitalize me please"
        );
    }

    #[test]
    fn pluralize_empty_string() {
        let modifier = PluralizeModifier {};
        assert_eq!(modifier.apply(""), "");
    }

    #[test]
    fn pluralize_s_ending() {
        let modifier = PluralizeModifier {};
        assert_eq!(modifier.apply("princess"), "princesses");
    }

    #[test]
    fn pluralize_h_ending() {
        let modifier = PluralizeModifier {};
        assert_eq!(modifier.apply("bush"), "bushes");
    }

    #[test]
    fn pluralize_x_ending() {
        let modifier = PluralizeModifier {};
        assert_eq!(modifier.apply("sphinx"), "sphinxes");
    }

    #[test]
    fn pluralize_y_ending_after_vowel() {
        let modifier = PluralizeModifier {};
        assert_eq!(modifier.apply("matey"), "mateys");
    }

    #[test]
    fn pluralize_y_ending_after_consonant() {
        let modifier = PluralizeModifier {};
        assert_eq!(modifier.apply("party"), "parties");
    }

    #[test]
    fn pluralize_other() {
        let modifier = PluralizeModifier {};
        assert_eq!(modifier.apply("word"), "words");
    }
}
