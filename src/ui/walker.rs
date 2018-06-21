use ui::ast::*;
use ui::visit::*;

pub fn walk_model(model: &UIModel, visitor: &mut Painter) {
    match model {
        UIModel::Component(ref comp) => visitor.visit_component(comp),
        UIModel::Composite(ref components) => {
            for component in components {
                visitor.visit_composite(component);
                walk_model(component, visitor);
            }
        }
    }
}
