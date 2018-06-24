
pub mod ast;
pub mod visit;
pub mod walker;

use ui::ast::UIComponent;
use ui::ast::UIModel;

use sdl2::rect::Rect;
use sdl2::pixels::Color;

pub fn create() -> UIModel {
    let model = UIModel::Composite(
        vec![
            Box::new(UIModel::Component(UIComponent::Background(Color::RGB(0x45, 0x45, 0x45)))),
            Box::new(UIModel::Component(UIComponent::Rectangle(Rect::new(0, 0, 200, 600), Color::RGB(0x39, 0x39, 0x39))))
        ]
    ); 
    model
}
