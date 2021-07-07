use crate::css::cssom::cssom::CSSOM;
use crate::html::dom::dom::DOMNode;
use crate::render_tree::render_object::RenderObject;

pub struct RenderTree {
    dom: DOMNode,
    cssom: CSSOM,
    tree: RenderObject
}

impl RenderTree {
    pub fn new(dom: DOMNode, cssom: CSSOM) -> Self {
        Self {
            dom,
            cssom,
            tree: RenderObject::new()
        }
    }

    pub fn constructor(&mut self) {
        
    }
}
