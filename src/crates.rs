use crate::enums::Node;
use crate::structs::Parser;
use crate::MARKDOWN_TOKENS;

impl Parser {
    pub fn new(input: String) -> Parser {
        Parser {
            pos: 0,
            input,
            tokens: Vec::new(),
        }
    }
    pub fn consume(&mut self, n: usize) {
        self.pos += n;
    }
    pub fn peek(&self, n: usize) -> Option<char> {
        self.input.chars().nth(self.pos + n)
    }
    pub fn peek_next(&self) -> Option<char> {
        self.peek(0)
    }
    pub fn get_char(&mut self) -> Option<char> {
        let c = self.peek_next();
        self.consume(1);
        c
    }
    pub fn roll_back(&mut self) {
        self.pos -= 1;
    }

    fn parse_heading(&mut self) -> Option<Node> {
        let mut level = 0;
        while let Some('#') = self.peek_next() {
            self.consume(1);
            level += 1;
        }
        if level == 0 {
            return None;
        }
        let mut nodes = Vec::new();
        while let Some(c) = self.peek_next() {
            if c == '\n' {
                break;
            }
            if let Some(&(name, _)) = MARKDOWN_TOKENS.iter().find(|&&(_, t)| t == c) {
                match name {
                    "link" => {
                        if let Some(node) = self.parse_link() {
                            nodes.push(node);
                        }
                    }
                    "image" => {
                        if let Some(node) = self.parse_image() {
                            nodes.push(node);
                        }
                    }
                    "code_block" => {
                        if let Some('`') = self.peek(1) {
                            if let Some(node) = self.parse_code_block() {
                                nodes.push(node);
                            }
                        } else {
                            if let Some(node) = self.parse_code_inline() {
                                nodes.push(node);
                            }
                        }
                    }
                    "strong_emphasis" => {
                        if let Some('*') = self.peek(1) {
                            self.consume(2);
                            if let Some(node) = self.parse_strong() {
                                nodes.push(node);
                            }
                        } else {
                            self.consume(1);
                            if let Some(node) = self.parse_emphasis() {
                                nodes.push(node);
                            } else if let Some(node) = self.parse_italic() {
                                nodes.push(node);
                            }
                        }
                    }
                    "italic_emphasis" => {
                        if let Some('~') = self.peek(1) {
                            self.consume(2);
                            if let Some(node) = self.parse_strikethrough() {
                                nodes.push(node);
                            }
                        } else {
                            self.consume(1);
                            if let Some(node) = self.parse_subscript() {
                                nodes.push(node);
                            }
                        }
                    }
                    "underline" => {
                        self.consume(1);
                        if let Some(node) = self.parse_underline() {
                            nodes.push(node);
                        }
                    }
                    "superscript" => {
                        self.consume(1);
                        if let Some(node) = self.parse_superscript() {
                            nodes.push(node);
                        }
                    }
                    "quote" => {
                        self.consume(1);
                        if let Some(node) = self.parse_quote() {
                            nodes.push(node);
                        }
                    }
                    "newline" => {
                        self.consume(1);
                        nodes.push(Node::Newline);
                    }
                    _ => {
                        if let Some(node) = self.parse_string() {
                            nodes.push(node);
                        }
                    }
                }
            } else {
                if let Some(node) = self.parse_string() {
                    nodes.push(node);
                }
            }
        }

        Some(Node::Heading(nodes, level))
    }
    fn parse_paragraph(&mut self) -> Option<Node> {
        let mut nodes = Vec::new();
        while let Some(c) = self.peek_next() {
            if c == '\n' {
                break;
            }
            if let Some(&(name, _)) = MARKDOWN_TOKENS.iter().find(|&&(_, t)| t == c) {
                match name {
                    "link" => {
                        if let Some(node) = self.parse_link() {
                            nodes.push(node);
                        }
                    }
                    "image" => {
                        if let Some(node) = self.parse_image() {
                            nodes.push(node);
                        }
                    }
                    "code_block" => {
                        if let Some('`') = self.peek(1) {
                            if let Some(node) = self.parse_code_block() {
                                nodes.push(node);
                            }
                        } else {
                            if let Some(node) = self.parse_code_inline() {
                                nodes.push(node);
                            }
                        }
                    }
                    "strong_emphasis" => {
                        if let Some('*') = self.peek(1) {
                            self.consume(2);
                            if let Some(node) = self.parse_strong() {
                                nodes.push(node);
                            }
                        } else {
                            self.consume(1);
                            if let Some(node) = self.parse_emphasis() {
                                nodes.push(node);
                            } else if let Some(node) = self.parse_italic() {
                                nodes.push(node);
                            }
                        }
                    }
                    "italic_emphasis" => {
                        if let Some('~') = self.peek(1) {
                            self.consume(2);
                            if let Some(node) = self.parse_strikethrough() {
                                nodes.push(node);
                            }
                        } else {
                            self.consume(1);
                            if let Some(node) = self.parse_subscript() {
                                nodes.push(node);
                            }
                        }
                    }
                    "underline" => {
                        self.consume(1);
                        if let Some(node) = self.parse_underline() {
                            nodes.push(node);
                        }
                    }
                    "superscript" => {
                        self.consume(1);
                        if let Some(node) = self.parse_superscript() {
                            nodes.push(node);
                        }
                    }
                    "quote" => {
                        self.consume(1);
                        if let Some(node) = self.parse_quote() {
                            nodes.push(node);
                        }
                    }
                    "newline" => {
                        self.consume(1);
                        nodes.push(Node::Newline);
                    }
                    _ => {
                        if let Some(node) = self.parse_string() {
                            nodes.push(node);
                        }
                    }
                }
            } else {
                if let Some(node) = self.parse_string() {
                    nodes.push(node);
                }
            }
        }

        if !nodes.is_empty() {
            Some(Node::Paragraph(nodes))
        } else {
            None
        }
    }
    fn parse_string(&mut self) -> Option<Node> {
        let mut text = String::new();
        while let Some(c) = self.peek_next() {
            if let Some(&(name, _)) = MARKDOWN_TOKENS.iter().find(|&&(_, t)| t == c) {
                match name {
                    "heading" => {
                        if let Some('#') = self.peek(1) {
                            break;
                        }
                    }
                    "link" => {
                        if let Some('[') = self.peek(1) {
                            break;
                        }
                    }
                    "image" => {
                        if let Some('!') = self.peek(1) {
                            break;
                        }
                    }
                    "code_block" => {
                        if let Some('`') = self.peek(1) {
                            break;
                        }
                    }
                    "strong_emphasis" => {
                        if let Some('*') = self.peek(1) {
                            break;
                        }
                    }
                    "italic_emphasis" => {
                        if let Some('~') = self.peek(1) {
                            break;
                        }
                    }
                    "underline" => {
                        if let Some('_') = self.peek(1) {
                            break;
                        }
                    }
                    "superscript" => {
                        if let Some('^') = self.peek(1) {
                            break;
                        }
                    }
                    "quote" => {
                        if let Some('>') = self.peek(1) {
                            break;
                        }
                    }
                    "newline" => {
                        if let Some('\n') = self.peek(1) {
                            break;
                        }
                    }
                    _ => {
                        if let Some(c) = self.get_char() {
                            text.push(c);
                        }
                    }
                }
            }
            text.push(c);
            self.consume(1);
        }
        Some(Node::String(text))
    }
    fn parse_link(&mut self) -> Option<Node> {
        let mut text = String::new();
        while let Some(c) = self.get_char() {
            if c == ']' {
                break;
            }
            text.push(c);
        }
        self.consume(1);
        self.consume(1);
        let mut link = String::new();
        while let Some(c) = self.get_char() {
            if c == ')' {
                break;
            }
            link.push(c);
        }
        Some(Node::Link(text, link))
    }
    fn parse_image(&mut self) -> Option<Node> {
        let mut text = String::new();
        while let Some(c) = self.get_char() {
            if c == ']' {
                break;
            }
            text.push(c);
        }
        self.consume(1);
        self.consume(1);
        let mut link = String::new();
        while let Some(c) = self.get_char() {
            if c == ')' {
                break;
            }
            link.push(c);
        }
        Some(Node::Image(text, link))
    }
    fn parse_code_inline(&mut self) -> Option<Node> {
        let mut text = String::new();
        while let Some(c) = self.get_char() {
            if c == '`' {
                break;
            }
            text.push(c);
        }
        Some(Node::CodeInline(text))
    }
    fn parse_code_block(&mut self) -> Option<Node> {
        let mut language = String::new();
        while let Some(c) = self.get_char() {
            if c == '\n' {
                break;
            }
            language.push(c);
        }
        let mut text = String::new();
        while let Some(c) = self.get_char() {
            if c == '`' {
                if let Some('`') = self.peek_next() {
                    if let Some('`') = self.peek(1) {
                        self.consume(3);
                        break;
                    }
                }
            }
            text.push(c);
        }
        Some(Node::CodeBlock(language, text))
    }
    fn parse_emphasis(&mut self) -> Option<Node> {
        let mut text = String::new();
        while let Some(c) = self.get_char() {
            if c == '*' {
                if let Some('*') = self.peek_next() {
                    self.consume(1);
                    break;
                }
            }
            text.push(c);
        }
        Some(Node::Emphasis(text))
    }
    fn parse_strong(&mut self) -> Option<Node> {
        let mut text = String::new();
        while let Some(c) = self.get_char() {
            if c == '*' {
                if let Some('*') = self.peek_next() {
                    self.consume(1);
                    break;
                }
            }
            text.push(c);
        }
        Some(Node::Strong(text))
    }
    fn parse_strikethrough(&mut self) -> Option<Node> {
        let mut text = String::new();
        while let Some(c) = self.get_char() {
            if c == '~' {
                if let Some('~') = self.peek_next() {
                    self.consume(1);
                    break;
                }
            }
            text.push(c);
        }
        Some(Node::Strikethrough(text))
    }
    fn parse_italic(&mut self) -> Option<Node> {
        let mut text = String::new();
        while let Some(c) = self.get_char() {
            if c == '*' {
                if let Some('*') = self.peek_next() {
                    self.consume(1);
                    break;
                }
            }
            text.push(c);
        }
        Some(Node::Italic(text))
    }
    fn parse_underline(&mut self) -> Option<Node> {
        let mut text = String::new();
        while let Some(c) = self.get_char() {
            if c == '_' {
                if let Some('_') = self.peek_next() {
                    self.consume(1);
                    break;
                }
            }
            text.push(c);
        }
        Some(Node::Underline(text))
    }
    fn parse_superscript(&mut self) -> Option<Node> {
        let mut text = String::new();
        while let Some(c) = self.get_char() {
            if c == '^' {
                if let Some('^') = self.peek_next() {
                    self.consume(1);
                    break;
                }
            }
            text.push(c);
        }
        Some(Node::Superscript(text))
    }
    fn parse_subscript(&mut self) -> Option<Node> {
        let mut text = String::new();
        while let Some(c) = self.get_char() {
            if c == '~' {
                if let Some('~') = self.peek_next() {
                    self.consume(1);
                    break;
                }
            }
            text.push(c);
        }
        Some(Node::Subscript(text))
    }
    fn parse_quote(&mut self) -> Option<Node> {
        let mut text = String::new();
        while let Some(c) = self.get_char() {
            if c == '>' {
                break;
            } else if MARKDOWN_TOKENS.iter().any(|(_, t)| *t == c) {
                self.get_char();
                break;
            }

            text.push(c);
        }
        Some(Node::Quote(text))
    }
    pub fn parse(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();
        while let Some(c) = self.peek_next() {
            if let Some(&(name, _)) = MARKDOWN_TOKENS.iter().find(|&&(_, t)| t == c) {
                match name {
                    "heading" => {
                        if let Some(node) = self.parse_heading() {
                            nodes.push(node);
                        }
                    }
                    "link" => {
                        if let Some(node) = self.parse_link() {
                            nodes.push(node);
                        }
                    }
                    "image" => {
                        if let Some(node) = self.parse_image() {
                            nodes.push(node);
                        }
                    }
                    "code_block" => {
                        if let Some('`') = self.peek(1) {
                            if let Some(node) = self.parse_code_block() {
                                nodes.push(node);
                            }
                        } else {
                            if let Some(node) = self.parse_code_inline() {
                                nodes.push(node);
                            }
                        }
                    }
                    "strong_emphasis" => {
                        if let Some('*') = self.peek(1) {
                            self.consume(2);
                            if let Some(node) = self.parse_strong() {
                                nodes.push(node);
                            }
                        } else {
                            self.consume(1);
                            if let Some(node) = self.parse_emphasis() {
                                nodes.push(node);
                            } else if let Some(node) = self.parse_italic() {
                                nodes.push(node);
                            }
                        }
                    }
                    "italic_emphasis" => {
                        if let Some('~') = self.peek(1) {
                            self.consume(2);
                            if let Some(node) = self.parse_strikethrough() {
                                nodes.push(node);
                            }
                        } else {
                            self.consume(1);
                            if let Some(node) = self.parse_subscript() {
                                nodes.push(node);
                            }
                        }
                    }
                    "underline" => {
                        self.consume(1);
                        if let Some(node) = self.parse_underline() {
                            nodes.push(node);
                        }
                    }
                    "superscript" => {
                        self.consume(1);
                        if let Some(node) = self.parse_superscript() {
                            nodes.push(node);
                        }
                    }
                    "quote" => {
                        self.consume(1);
                        if let Some(node) = self.parse_quote() {
                            nodes.push(node);
                        }
                    }
                    "newline" => {
                        self.consume(1);
                        nodes.push(Node::Newline);
                    }
                    _ => unreachable!(),
                }
            } else {
                if let Some(node) = self.parse_paragraph() {
                    nodes.push(node);
                }
            }
        }
        nodes
    }
}
