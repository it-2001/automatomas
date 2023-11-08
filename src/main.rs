use std::{
    ffi::{CStr, OsStr},
    io::Empty,
};

/*

    KDO SE TU SAKRA HRABAL V THEMES - Danecek

*/
mod cells;
use cells::*;

mod grid;
use grid::*;

use raylib::prelude::*;

const WINDOW_INIT_SIZE: (i32, i32) = (640, 480);
const GRID_INIT_SIZE: (i32, i32) = (192, 144);
const ITERATIONS: i32 = 15000;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_INIT_SIZE.0, WINDOW_INIT_SIZE.1)
        .resizable()
        .title("AutomaTomas")
        .build();

    let mut game = Game::new((rl.get_screen_width(), rl.get_screen_height()));

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let sys_time = std::time::SystemTime::now();
        if rl.is_window_resized() {
            game.resize_screen((rl.get_screen_width(), rl.get_screen_height()));
        }
        let mut d = rl.begin_drawing(&thread);

        game.update();

        d.clear_background(Color::WHITE);
        game.draw(&mut d);

        // draw fps in the bottom left corner
        d.draw_fps(10, game.screen.1 - 30);

        // print runtime usage
        // let elapsed = sys_time.elapsed().unwrap();
        // d.draw_text(&format!("{}ms", elapsed.as_millis()), 10, 30, 20, Color::BLACK);
    }
}

struct Game {
    grid: Grid,
    settings: Settings,
    screen: (i32, i32),
    saved: Vec<Grid>,
}

struct Settings {
    iterations: i32,
    pause: bool,
    brush: Brush,
}

struct Brush {
    size: i32,
    state: Option<CellStates>,
}

impl Game {
    pub fn new(screen: (i32, i32)) -> Game {
        let mut grid = Grid::new(GRID_INIT_SIZE);
        grid.cells[3][6] = Cell {
            state: CellStates::Sand,
        };
        grid.recalculate_dim(screen);
        Game {
            grid,
            screen,
            settings: Settings {
                iterations: ITERATIONS,
                pause: false,
                brush: Brush {
                    size: 3,
                    state: None,
                },
            },
            saved: Vec::new(),
        }
    }

    /// Resizes the screen and recalculates the grid's dimensions.
    pub fn resize_screen(&mut self, screen: (i32, i32)) {
        self.grid.recalculate_dim(screen);
        self.screen = screen;
    }

    pub fn update(&mut self) {
        if !self.settings.pause {
            for _ in 0..self.settings.iterations {
                self.grid.step();
            }
        }
    }

    /// Draws the game.
    ///
    /// This is also where the controls are handled. (since they need to be drawn)
    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        let mouse = d.get_mouse_position();
        let mouse_cell = (
            (mouse.x as f64 / self.grid.cell_dim.0 as f64
                - self.grid.dim.0 as f64 / self.grid.cell_dim.0 as f64) as i32,
            (mouse.y as f64 / self.grid.cell_dim.1 as f64
                - self.grid.dim.1 as f64 / self.grid.cell_dim.1 as f64) as i32,
        );
        // check bounds
        if d.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON)
            && mouse_cell.0 >= 0
            && mouse_cell.0 < self.grid.size.0
            && mouse_cell.1 >= 0
            && mouse_cell.1 < self.grid.size.1
        {
            match self.settings.brush.state {
                Some(state) => {
                    for x in 0..self.settings.brush.size {
                        for y in 0..self.settings.brush.size {
                            if mouse_cell.0 + x < self.grid.size.0
                                && mouse_cell.1 + y < self.grid.size.1
                            {
                                self.grid.cells[(mouse_cell.0 + x) as usize]
                                    [(mouse_cell.1 + y) as usize]
                                    .state = state;
                            }
                        }
                    }
                }
                None => (),
            }
        }
        macro_rules! cstr {
            ($s:expr) => {
                Some(std::ffi::CString::new($s).unwrap().as_c_str())
            };
            () => {
                None
            };
        }
        let button_dims: (i32, i32) = ((self.grid.dim.0 * 0.85) as i32, (self.grid.dim.1) as i32);
        let button_padding: (i32, i32) = (
            (self.grid.dim.0 * 0.05) as i32,
            (self.grid.dim.1 * 0.25) as i32,
        );
        let button_height = (self.grid.dim.1 * 1.2) as i32;
        // left side
        // draw controls
        let pause_txt = if self.settings.pause {
            "Resume"
        } else {
            "Pause"
        };
        if d.gui_button(
            Rectangle::new(
                button_padding.0 as f32,
                button_height as f32,
                button_dims.0 as f32,
                button_dims.1 as f32,
            ),
            cstr!(pause_txt),
        ) {
            self.settings.pause = !self.settings.pause;
        }
        if d.gui_button(
            Rectangle::new(
                button_padding.0 as f32,
                button_height as f32 + button_dims.1 as f32 * 1.05,
                button_dims.0 as f32,
                button_dims.1 as f32,
            ),
            cstr!("Clear"),
        ) {
            for x in 0..self.grid.size.0 {
                for y in 0..self.grid.size.1 {
                    self.grid.cells[x as usize][y as usize].state = CellStates::Air;
                }
            }
        }
        if d.gui_button(
            Rectangle::new(
                button_padding.0 as f32,
                button_height as f32 + button_dims.1 as f32 * 2.1,
                button_dims.0 as f32,
                button_dims.1 as f32,
            ),
            cstr!("Save"),
        ) {
            self.saved.push(self.grid.clone());
        }
        let load_txt = if self.saved.len() > 0 {
            format!("load ({})", self.saved.len())
        } else {
            format!("no saves")
        };
        if d.gui_button(
            Rectangle::new(
                button_padding.0 as f32,
                button_height as f32 + button_dims.1 as f32 * 3.15,
                button_dims.0 as f32,
                button_dims.1 as f32,
            ),
            cstr!(load_txt.as_str()),
        ) {
            if let Some(grid) = self.saved.pop() {
                self.grid = grid;
            }
        }
        // bounds, text_left, text_right, value, min_value, max_value
        self.settings.brush.size = d.gui_slider_bar(
            Rectangle::new(
                button_padding.0 as f32 + 33.,
                button_height as f32 + button_dims.1 as f32 * 4.2,
                button_dims.0 as f32 - 33.,
                button_dims.1 as f32,
            ),
            cstr!("Brush"),
            None,
            self.settings.brush.size as f32,
            1.,
            25.,
        ) as i32;
        // right side
        // draw button for each cell state for brush
        for (idx, state) in CellStates::list().iter().enumerate() {
            let button_txt = if self.settings.brush.state.is_some()
                && self.settings.brush.state.unwrap() == *state
            {
                format!("[X]-{state}")
            } else {
                format!("[ ]-{state}")
            };
            let button_pos = (
                self.grid.dim.2 as f32 + self.grid.dim.0 as f32 * 1.1,
                button_height as f32 + idx as f32 * button_dims.1 as f32 * 1.05,
            );
            if d.gui_button(
                Rectangle::new(
                    button_pos.0,
                    button_pos.1,
                    button_dims.0 as f32,
                    button_dims.1 as f32,
                ),
                cstr!(button_txt),
            ) {
                match self.settings.brush.state {
                    Some(s) => {
                        if s == *state {
                            self.settings.brush.state = None;
                        } else {
                            self.settings.brush.state = Some(*state);
                        }
                    }
                    None => {
                        self.settings.brush.state = Some(*state);
                    }
                }
            }
        }
        // grid
        self.grid.draw(d);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub state: CellStates,
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            state: CellStates::Air,
        }
    }
}
