mod renderer;
use renderer::Renderer;

use rand::Rng;

const WINDOW_WIDTH:u32 = 1920;
const WINDOW_HEIGHT:u32 = 1080;

fn main() {
    let mut renderer = Renderer::new("Raytracer", WINDOW_WIDTH, WINDOW_HEIGHT);

    let mut rng = rand::thread_rng();

    for x in 1..=WINDOW_WIDTH {
        for y in 1..=WINDOW_HEIGHT {
            let r:u8 = rng.gen_range(200..255);
            let g:u8 = rng.gen_range(0..155);
            let b:u8 = rng.gen_range(0..155);

            let color = (r, g, b);

            match renderer.put_pixel(color, (x as i32, y as i32)) {
                Ok(()) => (),
                Err(e) => panic!("{}", e),
            }

        }
        print!("\rRendering pixels: {} of {}", x * WINDOW_HEIGHT, WINDOW_WIDTH * WINDOW_HEIGHT);
    }
    println!("");

    renderer.present();

    renderer.keep_alive();
}
