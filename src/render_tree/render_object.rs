#[derive(Debug, PartialEq, Clone)]
pub struct _RenderObject {
    children: Vec<RenderObject>
}

#[derive(Debug, PartialEq, Clone)]
pub enum RenderObject {
    ViewPort(_RenderObject),
    Scroll(_RenderObject),
    Block(_RenderObject),
    Text(String)
}

impl RenderObject {
    pub fn new() -> Self {
        Self::ViewPort(_RenderObject {
            children: vec![]
        })
    }
}