
pub mod ast;
pub mod visit;

use ui::ast::UIComponent;
use ui::ast::UIModel;

use sdl2::rect::Rect;

pub fn create() -> UIModel {
    let model = UIModel::Composite(
        vec![
            Box::new(UIModel::Component(UIComponent::Rectangle(Rect::new(0, 0, 20, 20))))
        ]
    ); 
    model
}
