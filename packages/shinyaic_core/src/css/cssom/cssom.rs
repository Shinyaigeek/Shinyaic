use crate::css::cssom::declarations::Declarations;
use crate::css::cssom::selector::Selector;
use crate::html::dom::dom::DOMNode;

// Selector: #id .class tagname, Declaration HashSet<key, value> width: 100px, padding: auto
pub type CSSOM = Vec<StylingRule>;

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct StylingRule {
    pub selector: Vec<Selector>,
    pub declarations: Declarations,
}

impl StylingRule {
    pub fn matches(self, element: &DOMNode, parent_elm: &DOMNode) -> bool {
        for selector in self.selector {
            if selector.matches(element, parent_elm) {
                return true;
            }
        }
        return false;
    }
}

// CSSOM
// [Rule{...}, Rule{...}]

// RUle
// .class1, #id1 {
//     width: 100px;
//     height: 200px;
// }

// Selector
// .class1

// Declaration
// width: 100px;
