extern crate sdl2;

use sdl2::Sdl;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::event::Event;
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
            .position_centered()
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

    pub(crate) fn put_pixel<C>(&mut self, color: C, mut x: i32, mut y: i32) -> Result<(), String> 
    where
        C: Into<Color>,
    {
        let (w, h) = self.canvas.window().size();

        x = w as i32/2 + x;
        y = h as i32/2 - y;
        
        self.canvas.set_draw_color(color);
        let res = self.canvas.draw_point((x,y));
        //println!("pixel put at: ({}, {})", x, y);

        return res;
    }

    pub(crate) fn present(&mut self) {
        self.canvas.present();
    }

    pub(crate) fn keep_alive(&self) {
        loop {
            self.exit_events();
        }
    }

    pub(crate) fn get_canvas_size(&self) -> (u32, u32) {
        self.canvas.window().size()
    }

    pub(crate) fn exit_events(&self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        std::process::exit(0);
                    },
                _ => {}
            }
        }
    }
}
