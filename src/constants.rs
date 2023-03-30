use std::collections::HashMap;

pub const MARKDOWN_TOKENS: &[(&str, char)] = &[
    ("heading", '#'),
    ("emphasis", '*'),
    ("emphasis", '_'),
    ("strikethrough", '~'),
    ("strikethrough", '-'),
    ("strong", '*'),
    ("strong", '_'),
    ("code", '`'),
    ("link", '['),
    ("image", '!'),
    ("blockquote", '>'),
    ("unordered_list", '-'),
    ("unordered_list", '*'),
    ("table", '|'),
    ("footnote", '^'),
    ("horizontal_rule", '-'),
    ("horizontal_rule", '*'),
    ("horizontal_rule", '_'),
];

pub fn create_token_map() -> HashMap<&'static str, char> {
    let mut map = HashMap::new();
    for &(s, c) in MARKDOWN_TOKENS.iter() {
        map.insert(s, c);
    }
    map
}
