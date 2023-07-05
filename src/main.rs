mod pool;

use graphics::types::Color;
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::{Button, EventLoop, Key, MouseButton, MouseCursorEvent, PressEvent, ReleaseEvent};
use piston_window::PistonWindow as Window;

use pool::Pool;

const WIDTH: usize = 128;
const HEIGHT: usize = 72;
const PIXEL_PER_CELL: usize = 10;

#[derive(PartialEq, Eq)]
enum SelectedPoolStructure {
    None,
    Glider,
    Acorn,
}
impl Default for SelectedPoolStructure {
    fn default() -> Self {
        SelectedPoolStructure::None
    }
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    pool: Pool,
    window: Window,
    cursor: [f64; 2],
    paused: bool,
    mouse_button_pressed: Option<MouseButton>,
    selected_pool_structure: SelectedPoolStructure,
    percent_speed: u8,
}
impl Default for App {
    fn default() -> Self {
        App::new(128, 72)
    }
}

impl App {
    const MAX_FPS: u64 = 165;
    const SPEED_STEP: u64 = 10;

    fn new(width: u32, height: u32) -> App {
        let mut pool: Pool = Pool::new(width, height);
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
            cursor: Default::default(),
            paused: false,
            mouse_button_pressed: None,
            selected_pool_structure: Default::default(),
            percent_speed: 10,
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

    fn get_selected_pool(&self) -> Pool {
        match self.selected_pool_structure {
            SelectedPoolStructure::None => Pool::from_array(&[[true]]),
            SelectedPoolStructure::Glider => Pool::glider_south_east(),
            SelectedPoolStructure::Acorn => Pool::acorn(),
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const LIFE_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
        const DEAD_COLOR: Color = [1.0, 1.0, 1.0, 1.0];
        const HINT_COLOR: Color = [0.0, 0.0, 0.0, 0.5];
        const TEXT_COLOR: Color = [1.0, 0.1, 0.1, 1.0];

        let selected_pool = self.get_selected_pool();

        self.gl.draw(args.viewport(), |c, g| {
            // Clear the screen.
            clear(DEAD_COLOR, g);

            // Draw a square for each living cell
            for i in 0..HEIGHT {
                for j in 0..WIDTH {
                    if self.pool.get_cell(i as u32, j as u32) {
                        let (i_px, j_px) = Self::get_cell_pixel_coordinates(i as u32, j as u32);
                        rectangle(
                            LIFE_COLOR,
                            rectangle::square(0.0, 0.0, PIXEL_PER_CELL as f64),
                            c.transform.trans(j_px as f64, i_px as f64),
                            g,
                        );
                    }
                }
            }

            // Draw pixels about to be drawn with transparency
            let (selected_row, selected_column) = Self::cursor_to_cell_coordinates(self.cursor);
            for i in 0..selected_pool.height() {
                for j in 0..selected_pool.width() {
                    if selected_pool.get_cell(i, j) {
                        let hint_row = i + selected_row;
                        let hint_column = j + selected_column;
                        let (hint_row_px, hint_column_px) =
                            Self::get_cell_pixel_coordinates(hint_row, hint_column);
                        rectangle(
                            HINT_COLOR,
                            rectangle::square(0.0, 0.0, PIXEL_PER_CELL as f64),
                            c.transform.trans(hint_column_px as f64, hint_row_px as f64),
                            g,
                        );
                    }
                }
            }
            // TODO : render paused logo, key bindings

            let assets = find_folder::Search::ParentsThenKids(3, 3)
                .for_folder("assets")
                .expect("assets directory not found.");
            let ref font = assets.join("FiraSans-Regular.ttf");
            let mut glyph_cache = GlyphCache::new(font, (), TextureSettings::new()).unwrap();
            text::Text::new_color(TEXT_COLOR, 32)
                .draw(
                    "Hello opengl_graphics!",
                    &mut glyph_cache,
                    &DrawState::default(),
                    c.transform.trans(10.0, 100.0),
                    g,
                )
                .unwrap();
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
    fn handle_pressed_mouse(&mut self) {
        if let Some(pressed_button) = self.mouse_button_pressed {
            let (row, column) = Self::cursor_to_cell_coordinates(self.cursor);
            match pressed_button {
                MouseButton::Left => {
                    let struct_to_add = self.get_selected_pool();
                    let (row_offset, column_offset) = Self::cursor_to_cell_coordinates(self.cursor);
                    self.pool += struct_to_add.with_offset(row_offset, column_offset)
                }
                MouseButton::Right => self.pool.set_cell(row, column, false),
                _ => {}
            }
        }
    }

    /// Select given pool structure to be drawn if not already selected.
    /// If already selected deselects it.
    fn select_or_deselect_pool(&mut self, selected_pool: SelectedPoolStructure) {
        self.selected_pool_structure = if self.selected_pool_structure == selected_pool {
            SelectedPoolStructure::None
        } else {
            selected_pool
        }
    }

    fn process_keyboard(&mut self, key: Key, events: &mut Events) {
        match key {
            // Space : Pause / Resume when space is pressed
            Key::Space => self.paused = !self.paused,
            // Del : Clear pool
            Key::Delete => self.pool.clear(),
            // r : Randomize pool
            Key::R => self.pool.randomize(),
            // Right / Left : modify speed
            Key::Left => {
                if self.percent_speed > 0 {
                    self.percent_speed -= Self::SPEED_STEP as u8;
                }
                if self.percent_speed == 0 {
                    self.percent_speed = 1;
                }
                let new_update_per_second = Self::MAX_FPS * self.percent_speed as u64 / 100;
                events.set_ups(new_update_per_second);
            }
            Key::Right => {
                if self.percent_speed < 100 {
                    self.percent_speed += Self::SPEED_STEP as u8;
                }
                let new_update_per_second = Self::MAX_FPS * self.percent_speed as u64 / 100;
                events.set_ups(new_update_per_second);
            }
            // 1 : select glider
            Key::NumPad1 => self.select_or_deselect_pool(SelectedPoolStructure::Glider),
            // 2 : select acorn
            Key::NumPad2 => self.select_or_deselect_pool(SelectedPoolStructure::Acorn),

            // TODO : add a rotated field for the selected pool structure, to be able to put glider in other directions

            // Discard other keys
            _ => {}
        }
    }

    pub fn run(&mut self) {
        let update_per_second = Self::MAX_FPS * self.percent_speed as u64 / 100;
        let event_settings = EventSettings {
            max_fps: Self::MAX_FPS,
            ups: update_per_second,
            lazy: false,
            ..Default::default()
        };

        let mut events = Events::new(event_settings);
        //events.set_ups(10);
        while let Some(e) = events.next(&mut self.window) {
            // see https://docs.piston.rs/piston_window/piston_window/trait.EventLoop.html
            // This has a set_ups method

            // First capture mouse position.
            e.mouse_cursor(|pos| {
                self.cursor = pos.clone();
            });
            // Then process inputs.
            if let Some(Button::Mouse(button)) = e.press_args() {
                self.process_mouse_press(button);
            }
            if let Some(Button::Mouse(button)) = e.release_args() {
                self.process_mouse_release(button);
            }
            self.handle_pressed_mouse();
            if let Some(Button::Keyboard(key)) = e.press_args() {
                self.process_keyboard(key, &mut events);
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
