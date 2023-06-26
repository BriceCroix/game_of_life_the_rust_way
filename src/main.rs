mod pool;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

use pool::Pool;

const WIDTH: usize = 30;
const HEIGHT: usize = 15;
const PIXEL_PER_CELL: usize = 20;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    pool: Pool<WIDTH, HEIGHT>,
    window: Window,
}
impl Default for App {
    fn default() -> Self {
        let mut pool: Pool<WIDTH, HEIGHT> = Pool::new();
        pool.randomize();
        let opengl = OpenGL::V3_2;
        let window: Window = WindowSettings::new(
            "Game of life",
            [
                (WIDTH * PIXEL_PER_CELL) as u32,
                (HEIGHT * PIXEL_PER_CELL) as u32,
            ],
        )
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

        Self {
            gl: GlGraphics::new(OpenGL::V3_2),
            pool: pool,
            window: window,
        }
    }
}

impl App {
    fn get_cell_pixel_coordinates(row: u32, column: u32) -> (u32, u32) {
        (row * PIXEL_PER_CELL as u32, column * PIXEL_PER_CELL as u32)
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const LIFE_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const DEAD_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(DEAD_COLOR, gl);

            // Draw a square for each living cell
            for i in 0..HEIGHT {
                for j in 0..WIDTH {
                    if self.pool.get_cell(i as u32, j as u32) {
                        let (i_px, j_px) = Self::get_cell_pixel_coordinates(i as u32, j as u32);
                        rectangle(
                            LIFE_COLOR,
                            rectangle::square(0.0, 0.0, PIXEL_PER_CELL as f64),
                            c.transform.trans(j_px as f64, i_px as f64),
                            gl,
                        );
                    }
                }
            }
        });
    }

    pub fn update(&mut self, _args: &UpdateArgs) {
        self.pool.step();
    }

    pub fn run(&mut self) {
        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut self.window) {
            if let Some(args) = e.render_args() {
                self.render(&args);
            }

            if let Some(args) = e.update_args() {
                self.update(&args);
            }
        }
    }
}

fn main() {
    // Create a new game and run it.
    let mut app: App = Default::default();

    app.run();
}
