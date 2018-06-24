use ui::ast::*;
use ui::visit::*;

pub fn walk_model<T: Visitor>(model: &UIModel, visitor: &mut T) {
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
