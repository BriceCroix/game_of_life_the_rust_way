mod pool;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::{Button, Key, MouseButton, MouseCursorEvent, PressEvent};

use pool::Pool;

const WIDTH: usize = 128;
const HEIGHT: usize = 72;
const PIXEL_PER_CELL: usize = 10;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    pool: Pool<WIDTH, HEIGHT>,
    window: Window,
    paused: bool,
}
impl Default for App {
    fn default() -> Self {
        App::new()
    }
}

impl App {
    fn new() -> App {
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
            paused: false,
        }
    }

    fn get_cell_pixel_coordinates(row: u32, column: u32) -> (u32, u32) {
        (row * PIXEL_PER_CELL as u32, column * PIXEL_PER_CELL as u32)
    }

    fn cursor_to_cell_coordinates(cursor: [f64; 2]) -> (u32, u32) {
        (
            (cursor[1] / PIXEL_PER_CELL as f64) as u32,
            (cursor[0] / PIXEL_PER_CELL as f64) as u32,
        )
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
        if !self.paused {
            self.pool.step();
        }
    }

    fn process_mouse(&mut self, button: MouseButton, cursor_position: [f64; 2]) {
        match button {
            MouseButton::Left => {
                let (row, column) = Self::cursor_to_cell_coordinates(cursor_position);
                self.pool.set_cell(row, column, true);
            }
            MouseButton::Right => {
                let (row, column) = Self::cursor_to_cell_coordinates(cursor_position);
                self.pool.set_cell(row, column, false);
            }
            // Discard other buttons
            _ => {}
        }
    }

    fn process_keyboard(&mut self, key: Key) {
        match key {
            // Pause / Resume when space is pressed
            Key::Space => {
                self.paused = !self.paused;
            }
            // Discard other keys
            _ => {}
        }
    }

    pub fn run(&mut self) {
        const FPS: u64 = 10;
        let event_settings = EventSettings {
            max_fps: FPS,
            ups: FPS,
            ..Default::default()
        };

        let mut cursor = [0.0, 0.0];

        let mut events = Events::new(event_settings);
        while let Some(e) = events.next(&mut self.window) {
            // First capture mouse position.
            e.mouse_cursor(|pos| {
                cursor = pos.clone();
            });
            // Then process inputs.
            if let Some(Button::Mouse(button)) = e.press_args() {
                self.process_mouse(button, cursor);
            }
            if let Some(Button::Keyboard(key)) = e.press_args() {
                self.process_keyboard(key);
            };
            // Update state accordingly.
            if let Some(args) = e.update_args() {
                self.update(&args);
            }
            // Finally render.
            if let Some(args) = e.render_args() {
                self.render(&args);
            }
        }
    }
}

fn main() {
    // Create a new game and run it.
    let mut app: App = Default::default();

    app.run();
}
