pub const HTML_ELEMENT: &str = "html";
pub const BODY_ELEMENT: &str = "body";
pub const HEAD_ELEMENT: &str = "head";
pub const PARAGRAPH_ELEMENT: &str = "p";
pub const ANCHOR_ELEMENT: &str = "a";
pub const DIV_ELEMENT: &str = "div";

#[derive(Debug, PartialEq, Clone)]
pub enum HTMLElements {
    HTML_ELEMENT,
    BODY_ELEMENT,
    HEAD_ELEMENT,
    PARAGRAPH_ELEMENT,
    ANCHOR_ELEMENT,
    DIV_ELEMENT,
}
