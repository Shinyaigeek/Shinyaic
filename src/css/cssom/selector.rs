// TODO SelectorElmは正しいのか？, 属性selecytor
enum SelectorElm<'a> {
    id(&'a str),
    class(&'a str),
    tag_name(&'a str),
    asterisk(&'a str),
}

enum SelectorChildren<'a> {
    descendant_combinator(Vec<Selector<'a>>),
    child_combinator(Vec<Selector<'a>>),
    general_sibling_combinator(Vec<Selector<'a>>),
    adjacent_sibling_combinator(Vec<Selector<'a>>),
    column_combinator(Vec<Selector<'a>>),
}

pub struct Selector<'a> {
    elm: SelectorElm<'a>,
    children: Vec<SelectorChildren<'a>>,
}
// .class
// .class, .class2
// .class > .class2
