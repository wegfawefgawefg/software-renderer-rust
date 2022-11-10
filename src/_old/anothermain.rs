// import sdl2
use sdl2;

pub fn main(){
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut renderer = window
        .into_canvas()
        .build()
        .unwrap();


    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => {
                    break 'running
                },
                _ => {}
            }
        }

        // clear the screen
        renderer.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        renderer.clear();

        // draw a red rectangle
        renderer.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
        renderer.fill_rect(sdl2::rect::Rect::new(100, 100, 100, 100)).unwrap();

        // draw the screen
        renderer.present();
    }


}

