macro_rules! regex {
    ($regex:literal) => {{
        use once_cell::sync::Lazy;
        use regex::Regex;

        static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new($regex).unwrap());
        &*REGEX
    }};
}
