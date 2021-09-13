pub const HTML_ELEMENT: &str = "html";
pub const BODY_ELEMENT: &str = "body";
pub const HEAD_ELEMENT: &str = "head";
pub const PARAGRAPH_ELEMENT: &str = "p";
pub const ANCHOR_ELEMENT: &str = "a";
pub const DIV_ELEMENT: &str = "div";
pub const SPAN_ELEMENT: &str = "span";
pub const IMG_ELEMENT: &str = "img";
pub const TABLE_ELEMENT: &str = "table";
pub const TR_ELEMENT: &str = "tr";
pub const TD_ELEMENT: &str = "td";
pub const TH_ELEMENT: &str = "th";
pub const FORM_ELEMENT: &str = "form";
pub const INPUT_ELEMENT: &str = "input";
pub const BUTTON_ELEMENT: &str = "button";
pub const SELECT_ELEMENT: &str = "select";
pub const OPTION_ELEMENT: &str = "option";
pub const TITLE_ELEMENT: &str = "title";
pub const META_ELEMENT: &str = "meta";
pub const STYLE_ELEMENT: &str = "style";
pub const SCRIPT_ELEMENT: &str = "script";
pub const H1_ELEMENT: &str = "h1";

#[derive(Debug, PartialEq, Clone)]
pub enum HTMLElements {
    HTML_ELEMENT,
    BODY_ELEMENT,
    HEAD_ELEMENT,
    PARAGRAPH_ELEMENT,
    ANCHOR_ELEMENT,
    DIV_ELEMENT,
    SPAN_ELEMENT,
    IMG_ELEMENT,
    TABLE_ELEMENT,
    TR_ELEMENT,
    TD_ELEMENT,
    TH_ELEMENT,
    FORM_ELEMENT,
    INPUT_ELEMENT,
    BUTTON_ELEMENT,
    SELECT_ELEMENT,
    OPTION_ELEMENT,
    TITLE_ELEMENT,
    META_ELEMENT,
    STYLE_ELEMENT,
    SCRIPT_ELEMENT,
    H1_ELEMENT,
}

impl HTMLElements {
    pub fn to_string(self) -> String {
        let tag_str = match self {
            HTMLElements::HTML_ELEMENT => "html",
            HTMLElements::BODY_ELEMENT => "body",
            HTMLElements::HEAD_ELEMENT => "head",
            HTMLElements::PARAGRAPH_ELEMENT => "p",
            HTMLElements::ANCHOR_ELEMENT => "a",
            HTMLElements::DIV_ELEMENT => "div",
            HTMLElements::SPAN_ELEMENT => "span",
            HTMLElements::IMG_ELEMENT => "img",
            HTMLElements::TABLE_ELEMENT => "table",
            HTMLElements::TR_ELEMENT => "tr",
            HTMLElements::TD_ELEMENT => "td",
            HTMLElements::TH_ELEMENT => "th",
            HTMLElements::FORM_ELEMENT => "form",
            HTMLElements::INPUT_ELEMENT => "input",
            HTMLElements::BUTTON_ELEMENT => "button",
            HTMLElements::SELECT_ELEMENT => "select",
            HTMLElements::OPTION_ELEMENT => "option",
            HTMLElements::TITLE_ELEMENT => "title",
            HTMLElements::META_ELEMENT => "meta",
            HTMLElements::STYLE_ELEMENT => "style",
            HTMLElements::SCRIPT_ELEMENT => "script",
            HTMLElements::H1_ELEMENT => "h1",
        };

        tag_str.to_string()
    }

    pub fn need_closing_tag(&self) -> bool {
        match self {
            Self::META_ELEMENT => false,
            _ => true,
        }
    }
}
