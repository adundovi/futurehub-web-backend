use regex::Regex;
use deunicode::deunicode;

pub fn slugify(title: &String) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[^a-zA-Z\d]+").unwrap();
    }
    let title = deunicode(&title); // transiliterate to ASCII
    let title = RE.replace_all(&title, "-"); // replace all non-alphanumeric chars
    title
        .trim_matches('-') // remove front/back dashes
        .to_lowercase() // to lowercase
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slugify() {
        let title1 = "This is a title, Ž@{!".to_string();
        assert_eq!(slugify(&title1), "this-is-a-title-z".to_string());
        let title2 = "Nešto sasvim drugačije - _____?".to_string();
        assert_eq!(slugify(&title2),  "nesto-sasvim-drugacije".to_string());
        let title3 = "abcčćdđdžefghijklljmnnjoprsštuvzž".to_string();
        assert_eq!(slugify(&title3),  "abcccdddzefghijklljmnnjoprsstuvzz".to_string());
    }
}
