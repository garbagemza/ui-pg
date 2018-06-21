// The abstract visitor

use ui::ast::*;

pub trait Visitor {
    fn visit_component(&mut self, component: &UIComponent);
    fn visit_composite(&mut self, model: &UIModel);
}

pub struct Painter;

impl Visitor for Painter {
    fn visit_component(&mut self, component: &UIComponent) { println!("{}", component.value) }
    fn visit_composite(&mut self, _: &UIModel) { }
}

impl Painter {
    pub fn new() -> Painter {
        Painter {}
    }
}
