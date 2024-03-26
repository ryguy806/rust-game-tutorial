
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;

fn main() -> Result<(), String> {

    let screen_width: u32 = 800;
    let screen_height: u32 = 600;

    let sdl2_context = sdl2::init()?;
    let video_subsystem = sdl2_context.video()?;
    let window = video_subsystem.window("Hello World!", screen_width, screen_height).build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let screen_area = Rect::new(0, 0, screen_width, screen_height);
    let clear_color = Color::RGB(64, 192, 255);
    canvas.set_draw_color(clear_color);

    let mut running = true;
    let mut event_queue = sdl2_context.event_pump().unwrap();

    while running {
        for event in event_queue.poll_iter() {
            match event {
                Event::Quit {.. } => {
                    running = false;
                }
                Event::MouseMotion {
                    x, y, xrel, yrel,..
                } => {
                    println!("Mouse x: {}, y: {}", x, y);
                    println!("Relative x: {}, y: {}", xrel, yrel);
                } _ => {}
            }
        }

        let _ = canvas.fill_rect(screen_area);
        canvas.present();
    }

    Ok(())
}
