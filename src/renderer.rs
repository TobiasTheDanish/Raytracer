extern crate sdl2;

use sdl2::Sdl;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub(crate) struct Renderer {
    sdl_context: Sdl,
    canvas: Canvas<Window>,
}

impl Renderer {
    pub(crate) fn new(title: &str, width: u32, height: u32) -> Self {
        let sdl_context = match sdl2::init() {
            Ok(sdl) => sdl,
            Err(e) => panic!("{}", e),
        };

        let video = match sdl_context.video() {
            Ok(video) => video,
            Err(e) => panic!("{}", e),
        };

        let window = match video.window(title, width, height)
            .position(0, 0)
            .resizable()
            .build() {
                Ok(window) => window,
                Err(e) => panic!("{}", e),
            };

        let canvas = match window.into_canvas().build() {
            Ok(canvas) => canvas,
            Err(e) => panic!("{}", e),
        };

        Self {
            sdl_context,
            canvas,
        }
    }

    pub(crate) fn put_pixel<C, P>(&mut self, color: C, point: P) -> Result<(), String> 
    where
        C: Into<Color>,
        P: Into<Point>,
    {
            self.canvas.set_draw_color(color);
            self.canvas.draw_point(point)
    }

    pub(crate) fn present(&mut self) {
        self.canvas.present();
    }

    pub(crate) fn keep_alive(&self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();

        'wait: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                            break 'wait;
                        },
                    _ => {}
                }
            }
        }
    }
}
