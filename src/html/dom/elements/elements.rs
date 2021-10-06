#[derive(Debug, PartialEq, Clone)]
pub enum HTMLElements {
    HtmlElement,
    BodyElement,
    HeadElement,
    ParagraphElement,
    AnchorElement,
    DivElement,
    SpanElement,
    ImgElement,
    TableElement,
    TrElement,
    TdElement,
    ThElement,
    FormElement,
    InputElement,
    ButtonElement,
    SelectElement,
    OptionElement,
    TitleElement,
    MetaElement,
    StyleElement,
    ScriptElement,
    LinkElement,
    H1Element,
}

impl HTMLElements {
    pub fn to_string(self) -> String {
        let tag_str = match self {
            HTMLElements::HtmlElement => "html",
            HTMLElements::BodyElement => "body",
            HTMLElements::HeadElement => "head",
            HTMLElements::ParagraphElement => "p",
            HTMLElements::AnchorElement => "a",
            HTMLElements::DivElement => "div",
            HTMLElements::SpanElement => "span",
            HTMLElements::ImgElement => "img",
            HTMLElements::TableElement => "table",
            HTMLElements::TrElement => "tr",
            HTMLElements::TdElement => "td",
            HTMLElements::ThElement => "th",
            HTMLElements::FormElement => "form",
            HTMLElements::InputElement => "input",
            HTMLElements::ButtonElement => "button",
            HTMLElements::SelectElement => "select",
            HTMLElements::OptionElement => "option",
            HTMLElements::TitleElement => "title",
            HTMLElements::MetaElement => "meta",
            HTMLElements::StyleElement => "style",
            HTMLElements::ScriptElement => "script",
            HTMLElements::LinkElement => "link",
            HTMLElements::H1Element => "h1",
        };

        tag_str.to_string()
    }

    pub fn need_closing_tag(&self) -> bool {
        match self {
            Self::MetaElement => false,
            Self::LinkElement => false,
            Self::ImgElement => false,
            _ => true,
        }
    }

    pub fn init_from_str_tag(tag_str: &str) -> Option<Self> {
        match tag_str {
            "html" => Some(HTMLElements::HtmlElement),
            "body" => Some(HTMLElements::BodyElement),
            "head" => Some(HTMLElements::HeadElement),
            "p" => Some(HTMLElements::ParagraphElement),
            "a" => Some(HTMLElements::AnchorElement),
            "div" => Some(HTMLElements::DivElement),
            "span" => Some(HTMLElements::SpanElement),
            "img" => Some(HTMLElements::ImgElement),
            "table" => Some(HTMLElements::TableElement),
            "tr" => Some(HTMLElements::TrElement),
            "td" => Some(HTMLElements::TdElement),
            "th" => Some(HTMLElements::ThElement),
            "form" => Some(HTMLElements::FormElement),
            "input" => Some(HTMLElements::InputElement),
            "button" => Some(HTMLElements::ButtonElement),
            "select" => Some(HTMLElements::SelectElement),
            "option" => Some(HTMLElements::OptionElement),
            "title" => Some(HTMLElements::TitleElement),
            "meta" => Some(HTMLElements::MetaElement),
            "style" => Some(HTMLElements::StyleElement),
            "script" => Some(HTMLElements::ScriptElement),
            "link" => Some(HTMLElements::LinkElement),
            "h1" => Some(HTMLElements::H1Element),
            _ => None,
        }
    }
}
