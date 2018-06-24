use ui::ast::*;
use sdl2::render::Canvas;
use sdl2::video::Window;

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
            UIComponent::Rectangle(ref rect, ref color) => {
                self.canvas.set_draw_color(*color);
                self.canvas.fill_rect(*rect).expect("Unable to fill Rectangle!");
            }
            UIComponent::Background(ref color) => {
                self.canvas.set_draw_color(*color);
                self.canvas.clear();
            }
        };
    }
    fn visit_composite(&mut self, _: &UIModel) { }
}

impl<'a> Painter<'a> {
    pub fn new(canvas: &mut Canvas<Window>) -> Painter {
        Painter {
            canvas
        }
    }

    pub fn done(&mut self) {
        self.canvas.present();
    }
}
