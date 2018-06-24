
// The data we will visit
use sdl2::rect::Rect;
use sdl2::pixels::Color;

pub enum UIModel {
    Component(UIComponent),
    Composite(Vec<Box<UIModel>>)
}

pub enum UIComponent {
    Background(Color),
    Rectangle(Rect, Color)
}