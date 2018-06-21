
pub mod ast;
pub mod visit;

use ui::ast::UIComponent;
use ui::ast::UIModel;

pub fn create() -> UIModel {
    let model = UIModel::Composite(
        Box::new(UIModel::Component(UIComponent {value: "something".to_string()}))
    ); 
    model
}
