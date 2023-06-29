mod pool;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::{Button, Key, MouseButton, MouseCursorEvent, PressEvent, ReleaseEvent};

use pool::Pool;

const WIDTH: usize = 128;
const HEIGHT: usize = 72;
const PIXEL_PER_CELL: usize = 10;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    pool: Pool<WIDTH, HEIGHT>,
    window: Window,
    paused: bool,
    mouse_button_pressed: Option<MouseButton>,
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
        let window: Window = WindowSettings::new(
            "Game of life",
            [
                (WIDTH * PIXEL_PER_CELL) as u32,
                (HEIGHT * PIXEL_PER_CELL) as u32,
            ],
        )
        .graphics_api(OpenGL::V3_2)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

        Self {
            gl: GlGraphics::new(OpenGL::V3_2),
            pool: pool,
            window: window,
            paused: false,
            mouse_button_pressed: None,
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

    fn process_mouse_press(&mut self, button: MouseButton) {
        // Prevent press when already pressed
        if self.mouse_button_pressed != None {
            return;
        }
        self.mouse_button_pressed = Some(button)
    }

    fn process_mouse_release(&mut self, button: MouseButton) {
        // Prevent release when no button currently pressed
        if let Some(button_already_pressed) = self.mouse_button_pressed {
            if button != button_already_pressed {
                return;
            }
        }
        self.mouse_button_pressed = None;
    }

    /// Sets or kills cells depending on the button currently pressed on the mouse.
    fn handle_pressed_mouse(&mut self, cursor: [f64; 2]) {
        if let Some(pressed_button) = self.mouse_button_pressed {
            let (row, column) = Self::cursor_to_cell_coordinates(cursor);
            match pressed_button {
                MouseButton::Left => self.pool.set_cell(row, column, true),
                MouseButton::Right => self.pool.set_cell(row, column, false),
                _ => {}
            }
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
                self.process_mouse_press(button);
            }
            if let Some(Button::Mouse(button)) = e.release_args() {
                self.process_mouse_release(button);
            }
            self.handle_pressed_mouse(cursor);
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
