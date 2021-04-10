use crate::{Error, Result};

use regex::Regex;
use strsim::levenshtein as str_distance;


#[derive(Debug, Clone)]
pub enum Abbr {
    Literal(String),
    Wildcard,
}


impl Abbr {
    pub fn from_string(pattern: String) -> Result<Self> {
        let only_valid_re = Regex::new(r"^[\-_.a-zA-Z0-9]+$").unwrap();
        let only_dots_re = Regex::new(r"^\.+$").unwrap();

        // TODO: Forbid wildcards at last place?
        if pattern == "-" {
            Ok(Self::Wildcard)
        } else {
            if pattern.is_empty() {
                return Err(Error::InvalidSlice(pattern));
            }
            if !only_valid_re.is_match(&pattern) {
                return Err(Error::InvalidSlice(pattern));
            }
            if only_dots_re.is_match(&pattern) {
                return Err(Error::InvalidSlice(pattern));
            }


            Ok(Self::Literal(pattern.to_ascii_lowercase()))
        }
    }

    pub fn compare(&self, component: &str) -> Option<Congruence> {
        use Congruence::*;


        let component = component.to_ascii_lowercase();

        match self {
            Self::Literal(literal) =>
                if *literal == component.to_ascii_lowercase() {
                    Some(Complete)
                } else {
                    let mut abbr_chars = literal.chars().peekable();

                    for component_c in component.chars() {
                        match abbr_chars.peek() {
                            Some(abbr_c) => if *abbr_c == component_c.to_ascii_lowercase() {
                                abbr_chars.next(); // Consume char.
                            },
                            None => break,
                        }
                    }

                    let whole_abbr_consumed = abbr_chars.peek().is_none();

                    if whole_abbr_consumed {
                        let distance = str_distance(literal, &component);

                        Some(Partial(distance))
                    } else {
                        None
                    }
                }
            Self::Wildcard => Some(Wildcard),
        }
    }
}


#[derive(Clone, Debug)]
pub enum Congruence {
    Partial(usize),
    Wildcard,
    Complete,
}


#[cfg(test)]
mod test {
    use super::*;
    use Congruence::*;

    #[test]
    fn test_from_string() {
        let abbr = |s: &str| Abbr::from_string(s.to_string());

        assert!(abbr(".").is_err());
        assert!(abbr("..").is_err());
        assert!(abbr("...").is_err());
        assert!(abbr("one two three").is_err());


        let abbr = |s: &str| Abbr::from_string(s.to_string()).unwrap();

        variant!(abbr("-"), Abbr::Wildcard);
        variant!(abbr("mOcKiNgBiRd"), Abbr::Literal(literal) if literal == "mockingbird");
        variant!(abbr("X.ae.A-12"), Abbr::Literal(literal) if literal == "x.ae.a-12");

        // TODO
        // assert!(variant!(abbr("zażółć"), Abbr::Literal(literal) => literal
        // == "zażółć"));
    }


    #[test]
    fn test_wildcard_match() {
        let abbr = Abbr::Wildcard;


        variant!(abbr.compare("iks"), Some(Wildcard));
        variant!(abbr.compare("eh--ehe123"), Some(Wildcard));
    }


    #[test]
    fn test_literal_match() {
        let abbr = Abbr::Literal("mi".to_string());


        variant!(abbr.compare("mi"), Some(Complete));
        variant!(abbr.compare("Mi"), Some(Complete));
        variant!(abbr.compare("ooo..oo---mi-ooooo"), Some(Partial(_)));
        variant!(abbr.compare("ooo..oo---mI-ooooo"), Some(Partial(_)));
        variant!(abbr.compare("xxxxxx"), None);
    }


    #[test]
    fn test_subseries_match() {
        let abbr = Abbr::Literal("mi".to_string());


        let dist_a = variant!(abbr.compare("m-----i"), Some(Partial(dist_a)) => dist_a);
        let dist_b = variant!(abbr.compare("M--i"), Some(Partial(dist_b)) => dist_b);
        assert!(dist_a > dist_b);
        variant!(abbr.compare("im"), None);
    }
}
