
// The data we will visit
use sdl2::rect::Rect;

pub enum UIModel {
    Component(UIComponent),
    Composite(Vec<Box<UIModel>>)
}

pub enum UIComponent {
    Rectangle(Rect)
}