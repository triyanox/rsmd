#[derive(Debug, Clone)]
pub struct NodeVec {
    pub nodes: Vec<Node>,
}

#[derive(Debug, Clone)]
pub enum Node {
    Heading(Vec<Node>, u8),
    Paragraph(Vec<Node>),
    CodeBlock(String, String),
    CodeInline(String),
    Link(String, String),
    Image(String, String),
    Emphasis(String),
    Strong(String),
    Strikethrough(String),
    Italic(String),
    Underline(String),
    Superscript(String),
    Subscript(String),
    Quote(String),
    OrderedList(Vec<Vec<Node>>),
    UnorderedList(Vec<Vec<Node>>),
    Newline,
    String(String),
}
