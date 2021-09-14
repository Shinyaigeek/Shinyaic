pub const HtmlElement: &str = "html";
pub const BodyElement: &str = "body";
pub const HeadElement: &str = "head";
pub const ParagraphElement: &str = "p";
pub const AnchorElement: &str = "a";
pub const DivElement: &str = "div";
pub const SpanElement: &str = "span";
pub const ImgElement: &str = "img";
pub const TableElement: &str = "table";
pub const TrElement: &str = "tr";
pub const TdElement: &str = "td";
pub const ThElement: &str = "th";
pub const FormElement: &str = "form";
pub const InputElement: &str = "input";
pub const ButtonElement: &str = "button";
pub const SelectElement: &str = "select";
pub const OptionElement: &str = "option";
pub const TitleElement: &str = "title";
pub const MetaElement: &str = "meta";
pub const StyleElement: &str = "style";
pub const ScriptElement: &str = "script";
pub const H1Element: &str = "h1";

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
            HTMLElements::H1Element => "h1",
        };

        tag_str.to_string()
    }

    pub fn need_closing_tag(&self) -> bool {
        match self {
            Self::MetaElement => false,
            _ => true,
        }
    }
}
