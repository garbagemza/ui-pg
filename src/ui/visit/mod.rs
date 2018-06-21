// The abstract visitor

use ui::ast::*;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;

pub trait Visitor {
    fn visit_component(&mut self, component: &UIComponent);
    fn visit_composite(&mut self, model: &UIModel);
}

pub struct Painter<'a> {
    canvas: &'a mut Canvas<Window>
}

impl<'a> Visitor for Painter<'a> {
    fn visit_component(&mut self, component: &UIComponent) { 
        match component {
            UIComponent::Rectangle(ref rect) => {
                self.canvas.set_draw_color(Color::RGB(255, 210, 0));
                // A draw a rectangle which almost fills our window with it !
                self.canvas.fill_rect(*rect).expect("Unable to fill Rectangle!");
            }
        };
    }
    fn visit_composite(&mut self, _: &UIModel) { }
}

impl<'a> Painter<'a> {
    pub fn new(canvas: &mut Canvas<Window>) -> Painter {
        Painter {
            canvas: canvas
        }
    }
}
