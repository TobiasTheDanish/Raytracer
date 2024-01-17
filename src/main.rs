extern crate glam;

mod renderer;
mod scene;
mod raytracer;

use glam::Vec3;
use renderer::Renderer;
use scene::Scene;

const WINDOW_WIDTH:i32 = 1600;
const WINDOW_HEIGHT:i32 = 1200;

fn main() {
    let scene = Scene::load("./src/scenes/scene1.json");
    let renderer = Renderer::new("Raytracer", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32);
    let mut context = Context::init(scene, renderer);

    let origin = context.get_origin();
    let mut iters: i32 = 0;

    for x in -(WINDOW_WIDTH/2)..(WINDOW_WIDTH/2) {
        for y in -(WINDOW_HEIGHT/2)..(WINDOW_HEIGHT/2) {
            let ray_dir:Vec3 = context.canvas_to_viewport(x, y);
            let color = raytracer::trace_ray(&context, origin, ray_dir, 1.0, f32::INFINITY);

            match context.put_pixel(color, x, y) {
                Ok(()) => (),
                Err(e) => panic!("{}", e),
            }
            iters += 1;
        }

        print!("\rRendering pixels: {} of {}", iters, WINDOW_WIDTH * WINDOW_HEIGHT);

        context.exit_events();
    }
    println!("");

    context.present();

    context.keep_alive();
}

struct Context {
    scene: Scene,
    renderer: Renderer,
}

impl Context {
    fn init(scene: Scene, renderer: Renderer) -> Self {
        Self {
            scene,
            renderer,
        }
    }

    fn get_background(&self) -> (u8, u8, u8) {
        self.scene.get_background()
    }

    fn get_origin(&self) -> glam::Vec3 {
        self.scene.get_origin().into()
    }

    fn canvas_to_viewport(&self,x: i32, y: i32) -> glam::Vec3 {
        let (v_w, v_h, v_d) = self.scene.get_viewport_info();
        let (c_w, c_h) = self.renderer.get_canvas_size();

        let v_x = x as f32 * v_w/c_w as f32;
        let v_y = y as f32 * v_h/c_h as f32;

        (v_x, v_y, v_d).into()
    }

    fn put_pixel<C>(&mut self, color: C, x: i32, y: i32) -> Result<(), String> 
    where
        C: Into<sdl2::pixels::Color>
    {
        self.renderer.put_pixel(color, x, y)
    }

    fn present(&mut self) {
        self.renderer.present();
    }

    fn keep_alive(&self) {
        self.renderer.keep_alive();
    }

    fn exit_events(&self) {
        self.renderer.exit_events();
    }
}
