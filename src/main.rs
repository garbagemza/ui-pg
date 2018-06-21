extern crate sdl2;
mod ui;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::Sdl;

use ui::ast::UIModel;

use ui::visit::Visitor;
use ui::visit::Painter;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("ui-pg", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .accelerated()
        .present_vsync()
        .build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    
    let mut model = ui::create();
    let mut visitor = Painter::new(&mut canvas);
    main_loop(&sdl_context, &mut model, &mut visitor);
}

fn main_loop(context: &Sdl, mut model: &mut UIModel, mut visitor: &mut Painter) {
    let mut event_pump = context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        draw(&mut model, &mut visitor);
    }
}

fn draw<'a>(model: &mut UIModel, mut visitor: &mut Painter) {
    walk_model(model, &mut visitor);
    visitor.done();
}

fn walk_model(model: &UIModel, visitor: &mut Painter) {
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