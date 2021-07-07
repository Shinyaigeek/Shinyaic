// TODO SelectorElmは正しいのか？, 属性selecytor
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum SelectorElm {
    id(String),
    class(String),
    tag_name(String),
    asterisk(String),
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum SelectorChildren {
    descendant_combinator(Vec<Selector>),
    child_combinator(Vec<Selector>),
    general_sibling_combinator(Vec<Selector>),
    adjacent_sibling_combinator(Vec<Selector>),
    column_combinator(Vec<Selector>),
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct Selector {
    pub elm: SelectorElm,
    pub children: Vec<SelectorChildren>,
}
// .class
// .class, .class2
// .class > .class2
