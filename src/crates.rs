use crate::enums::Node;
use crate::structs::Parser;
use crate::{create_token_map, MARKDOWN_TOKENS};

impl Parser {
    pub fn new(input: String) -> Parser {
        Parser {
            pos: 0,
            input,
            tokens: Vec::new(),
        }
    }
    fn consume(&mut self, n: usize) {
        self.pos += n;
    }
    fn peek(&self, n: usize) -> Option<char> {
        self.input.chars().nth(self.pos + n)
    }
    fn peek_next(&self) -> Option<char> {
        self.peek(0)
    }
    fn get_char(&mut self) -> Option<char> {
        let c = self.peek_next();
        self.consume(1);
        c
    }
    fn roll_back(&mut self) {
        self.pos -= 1;
    }
    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }
    fn peek_next_n(&self, n: usize) -> Option<&str> {
        if self.pos + n <= self.input.len() {
            Some(&self.input[self.pos..self.pos + n])
        } else {
            None
        }
    }
    fn consume_until_newline(&mut self) {
        while let Some(c) = self.get_char() {
            if c == '\n' {
                break;
            }
        }
    }
    pub fn parse_node(&mut self) -> Option<Node> {
        if let Some(node) = self.parse_strikethrough() {
            return Some(node);
        }
        if let Some(node) = self.parse_emphasis() {
            return Some(node);
        }
        if let Some(node) = self.parse_code() {
            return Some(node);
        }
        if let Some(node) = self.parse_image() {
            return Some(node);
        }
        if let Some(node) = self.parse_link() {
            return Some(node);
        }
        if let Some(node) = self.parse_blockquote() {
            return Some(node);
        }
        if let Some(node) = self.parse_string() {
            return Some(node);
        }
        None
    }
    fn starts_with_numeric(&self) -> bool {
        if let Some(c) = self.peek_next() {
            c.is_numeric()
        } else {
            false
        }
    }

    pub fn parse_string(&mut self) -> Option<Node> {
        let mut content = String::new();
        while let Some(c) = self.get_char() {
            if MARKDOWN_TOKENS.iter().any(|&(_, ch)| ch == c) {
                self.roll_back();
                break;
            }
            content.push(c);
        }
        if !content.is_empty() {
            Some(Node::String(content))
        } else {
            None
        }
    }

    pub fn parse_strikethrough(&mut self) -> Option<Node> {
        if self.starts_with("--") {
            self.parse_hyphen_strikethrough()
        } else if self.starts_with("~~") {
            self.parse_tilde_strikethrough()
        } else {
            None
        }
    }
    fn parse_hyphen_strikethrough(&mut self) -> Option<Node> {
        if self.starts_with("--") {
            self.consume(2);
            let mut content = String::new();
            while let Some(c) = self.get_char() {
                if self.starts_with("--") {
                    self.consume(2); // consume second hyphen
                    return Some(Node::Strikethrough(content));
                }
                content.push(c);
            }
            Some(Node::Strikethrough(content)) // unclosed strikethrough
        } else {
            None
        }
    }
    fn parse_tilde_strikethrough(&mut self) -> Option<Node> {
        self.consume(2);
        let mut content = String::new();
        while let Some(c) = self.get_char() {
            if self.starts_with("~~") {
                self.consume(2);
                return Some(Node::Strikethrough(content));
            }
            content.push(c);
        }
        Some(Node::Strikethrough(content))
    }

    pub fn parse_emphasis(&mut self) -> Option<Node> {
        if let Some('*') = self.peek_next() {
            if self.peek_next_n(2) == Some("**") {
                return self.parse_strong_asterisk();
            } else {
                return self.parse_emphasis_asterisk();
            }
        } else if let Some('_') = self.peek_next() {
            if self.peek_next_n(2) == Some("__") {
                return self.parse_strong_underscore();
            } else {
                return self.parse_emphasis_underscore();
            }
        } else {
            None
        }
    }
    fn parse_emphasis_asterisk(&mut self) -> Option<Node> {
        self.consume(1);
        let mut content = String::new();
        while let Some(c) = self.get_char() {
            if c == '*' && self.peek_next() == Some('*') {
                self.consume(1);
                return Some(Node::Strong(content));
            } else if c == '*' {
                return Some(Node::Emphasis(content));
            }
            content.push(c);
        }
        Some(Node::Emphasis(content))
    }
    fn parse_strong_asterisk(&mut self) -> Option<Node> {
        self.consume(2);
        let mut content = String::new();
        while let Some(c) = self.get_char() {
            if c == '*' && self.peek_next() == Some('*') {
                self.consume(2);
                return Some(Node::Strong(content));
            }
            content.push(c);
        }
        Some(Node::Strong(content))
    }
    fn parse_emphasis_underscore(&mut self) -> Option<Node> {
        self.consume(1);
        let mut content = String::new();
        while let Some(c) = self.get_char() {
            if c == '_' && self.peek_next() == Some('_') {
                self.consume(1);
                return Some(Node::Strong(content));
            } else if c == '_' {
                return Some(Node::Emphasis(content));
            }
            content.push(c);
        }
        Some(Node::Emphasis(content))
    }
    fn parse_strong_underscore(&mut self) -> Option<Node> {
        self.consume(2);
        let mut content = String::new();
        while let Some(c) = self.get_char() {
            if c == '_' && self.peek_next() == Some('_') {
                self.consume(2);
                return Some(Node::Strong(content));
            }
            content.push(c);
        }
        Some(Node::Strong(content))
    }

    pub fn parse_code(&mut self) -> Option<Node> {
        if let Some('`') = self.peek_next() {
            self.parse_inline_code()
        } else if self.starts_with("```") {
            self.parse_code_block()
        } else {
            None
        }
    }
    fn parse_inline_code(&mut self) -> Option<Node> {
        self.consume(1);
        let mut content = String::new();
        while let Some(c) = self.get_char() {
            if c == '`' {
                return Some(Node::CodeInline(content));
            }
            content.push(c);
        }
        Some(Node::CodeInline(content))
    }
    fn parse_code_block(&mut self) -> Option<Node> {
        self.consume(3); // consume ```
        let lang = self.parse_language();
        self.consume_until_newline();
        let mut content = String::new();
        while let Some(c) = self.get_char() {
            if self.starts_with("```") {
                self.consume(3);
                return match lang {
                    Some(lang_str) => Some(Node::CodeBlockWithLang(content, lang_str)),
                    None => Some(Node::CodeBlock(content)),
                };
            }
            content.push(c);
        }
        match lang {
            Some(lang_str) => Some(Node::CodeBlockWithLang(content, lang_str)),
            None => Some(Node::CodeBlock(content)),
        }
    }
    fn parse_language(&mut self) -> Option<String> {
        let start_pos = self.pos;
        while let Some(c) = self.get_char() {
            if c.is_whitespace() {
                return Some(self.input[start_pos..self.pos].to_string());
            } else if c == '`' {
                return None;
            }
        }
        None
    }

    pub fn parse_link(&mut self) -> Option<Node> {
        if self.starts_with("[") {
            self.consume(1);

            let mut text = String::new();
            let mut url = String::new();

            while let Some(c) = self.get_char() {
                if c == ']' {
                    break;
                } else {
                    text.push(c);
                }
            }

            if self.starts_with("(") {
                self.consume(1);

                while let Some(c) = self.get_char() {
                    if c == ')' {
                        break;
                    } else {
                        url.push(c);
                    }
                }

                return Some(Node::Link(text, url));
            }
        }

        None
    }

    pub fn parse_image(&mut self) -> Option<Node> {
        if self.starts_with("!") && self.peek_next_n(2) == Some("[]") {
            self.consume(2);

            let mut alt_text = String::new();
            while let Some(c) = self.get_char() {
                if c == ']' {
                    break;
                } else {
                    alt_text.push(c);
                }
            }

            if self.starts_with("(") {
                self.consume(1);

                let mut url = String::new();
                while let Some(c) = self.get_char() {
                    if c == ')' {
                        break;
                    } else {
                        url.push(c);
                    }
                }
                return Some(Node::Image(alt_text, url));
            }
        }

        None
    }

    pub fn parse_blockquote(&mut self) -> Option<Node> {
        if self.starts_with(">")
            && (self.peek_next().is_none() || self.peek_next().unwrap().is_whitespace())
        {
            self.consume(1);

            let mut content = String::new();
            while let Some(c) = self.get_char() {
                if c == '\n' {
                    break;
                } else {
                    content.push(c);
                }
            }

            let mut children = Vec::new();
            let mut child_parser = Parser::new(content);
            while let Some(node) = child_parser.parse_node() {
                children.push(node);
            }

            return Some(Node::Blockquote(children));
        }

        None
    }

    pub fn parse_ordered_list(&mut self) -> Option<Node> {
        if let Some(n) = self.parse_list_item_count() {
            let mut items = vec![self.parse_list_item().unwrap()];
            while let Some(next) = self.parse_list_item() {
                items.push(next);
            }
            return Some(Node::OrderedList(items));
        }

        None
    }
    pub fn parse_unordered_list(&mut self) -> Option<Vec<Vec<Node>>> {
        if !self.starts_with("- ") && !self.starts_with("* ") {
            return None;
        }

        let mut items = vec![self.parse_list_item().unwrap()];
        while let Some(next) = self.parse_list_item() {
            items.push(next);
        }
        Some(items)
    }
    fn parse_list_item_count(&mut self) -> Option<usize> {
        let mut count_str = String::new();
        while self.starts_with_numeric() {
            count_str.push(self.get_char().unwrap());
        }
        if count_str.is_empty() {
            None
        } else {
            Some(count_str.parse().unwrap())
        }
    }
    fn parse_list_item(&mut self) -> Option<Vec<Node>> {
        if !self.starts_with("   ") && !self.starts_with("\t") {
            return None;
        }

        let mut content = String::new();
        while let Some(c) = self.get_char() {
            if c == '\n' {
                break;
            } else {
                content.push(c);
            }
        }

        let mut children = Vec::new();
        let mut child_parser = Parser::new(content);
        while let Some(node) = child_parser.parse_node() {
            children.push(node);
        }

        Some(children)
    }

    pub fn parse_paragraph(&mut self) -> Option<Node> {
        let mut nodes = Vec::new();
        let mut current_text = String::new();

        while let Some(c) = self.get_char() {
            if c == '\n' && self.peek_next() == Some('\n') {
                self.consume(1);
                break;
            } else if c == '\n' {
                current_text.push(' ');
            } else {
                if let Some(node) = self.parse_node() {
                    if !current_text.is_empty() {
                        nodes.push(Node::String(current_text.clone()));
                        current_text.clear();
                    }
                    nodes.push(node);
                } else {
                    current_text.push(c);
                }
            }
        }
        if !current_text.is_empty() {
            nodes.push(Node::String(current_text));
        }
        if nodes.is_empty() {
            None
        } else {
            Some(Node::Paragraph(nodes))
        }
    }

    pub fn parse_heading(&mut self) -> Option<Node> {
        let mut level = 0;
        while self.starts_with("#") {
            level += 1;
            self.consume(1);
        }

        if level == 0 {
            return None;
        }

        let mut nodes = Vec::new();
        while let Some(c) = self.get_char() {
            if c == '\n' {
                self.consume(1);
                break;
            } else {
                if let Some(node) = self.parse_node() {
                    nodes.push(node);
                }
            }
        }

        Some(Node::Heading(nodes, level))
    }
}
