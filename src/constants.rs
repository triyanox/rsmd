pub const MARKDOWN_TOKENS: &[(&str, char)] = &[
    ("heading", '#'),
    ("link", '['),
    ("image", '!'),
    ("code_block", '`'),
    ("strong_emphasis", '*'),
    ("italic_emphasis", '_'),
    ("strikethrough", '~'),
    ("subscript", '~'),
    ("underline", '_'),
    ("superscript", '^'),
    ("quote", '>'),
    ("newline", '\n'),
];
