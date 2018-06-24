extern crate sdl2;
mod ui;

use std::time::Instant;
use std::thread;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::Sdl;

use ui::ast::UIModel;
use ui::visit::Painter;
use ui::walker;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("ui-pg", 800, 600)
        .position_centered()
        .resizable()
        .opengl()
        .build()
        .unwrap();

    let canvas = window.into_canvas()
        .accelerated()
        .present_vsync()
        .build().unwrap();
    
    let mut model = ui::create();
    let mut visitor = Painter::new(canvas);
    main_loop(&sdl_context, &mut model, &mut visitor);
}

fn main_loop(context: &Sdl, model: &UIModel, mut visitor: &mut Painter) {
    let mut event_pump = context.event_pump().unwrap();
    'running: loop {
        let start = Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        walker::walk_model(model, &mut visitor);
        visitor.done();
        sleep_exceeded_time(start);
    }
}


fn sleep_exceeded_time(start_time: Instant) {
    let elapsed = start_time.elapsed();
    let diff_nanos = (1000_000_000.0 / 60.0) - (elapsed.subsec_nanos() as f64);
    if diff_nanos > 0.0 {
        thread::sleep(Duration::from_millis((diff_nanos / 1000_000.0) as u64));
    }
}

