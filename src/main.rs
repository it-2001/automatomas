use std::io::Empty;

/*

    KDO SE TU SAKRA HRABAL V THEMES - Danecek

*/
use raylib::prelude::*;
use rand::prelude::*;

const WINDOW_INIT_SIZE: (i32, i32) = (640, 480);
const GRID_INIT_SIZE: (i32, i32) = (50, 50);

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_INIT_SIZE.0, WINDOW_INIT_SIZE.1)
        .resizable()
        .msaa_4x()
        .title("Hello, World")
        .build();

    let mut grid = Grid::new(GRID_INIT_SIZE);
    let mut win_size = (rl.get_screen_width(), rl.get_screen_height());
    grid.recalculate_dim(win_size);

    grid.cells[3][6] = Cell{ state: CellStates::Sand};

    while !rl.window_should_close() {
        if rl.is_window_resized() {
            win_size = (rl.get_screen_width(), rl.get_screen_height());
            grid.recalculate_dim(win_size);
        }
        let mut d = rl.begin_drawing(&thread);
        
        d.clear_background(Color::WHITE);
        grid.draw(&mut d);
    }
}

#[derive(Clone, Debug)]
struct Grid {
    cells: Vec<Vec<Cell>>,
    rng: ThreadRng,
    size: (i32, i32),
    dim: (i32, i32, i32, i32),
    cell_dim: (i32, i32),
}

impl Grid {
    fn new((width, height): (i32, i32)) -> Grid {
        let mut cells = Vec::with_capacity(width as usize);
        for _ in 0..width {
            cells.push(vec![Cell::new(); height as usize]);
        }
        let rng = rand::thread_rng();

        Grid {
            cells,
            rng,
            size: (width, height),
            dim: (0,0,0,0),
            cell_dim: (0, 0),
        }
    }

    fn step(&mut self) {
        let cell = (self.rng.gen_range(0..self.size.0), self.rng.gen_range(0..self.size.1));
        match self.cells[cell.0 as usize][cell.1 as usize].state {
            CellStates::Empty => (),
            CellStates::Sand => {
                match self.get_cell(cell.0, cell.1 + 1) {
                    Some(other) => {
                        self.cells[cell.0 as usize][cell.1 as usize].state = other.state;
                        self.cells[cell.0 as usize][cell.1 as usize + 1 as usize].state = CellStates::Sand;
                    }
                    None => ()
                }
            },
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        const COLOR: CellStates = CellStates::Empty;
        d.draw_rectangle(self.dim.0, self.dim.1, self.dim.2, self.dim.3, CellStates::color(&COLOR));
        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
                let state = self.cells[x as usize][y as usize].state;
                d.draw_rectangle(self.dim.0 + x*self.cell_dim.0, self.dim.1 + y*self.cell_dim.1, self.cell_dim.0, self.cell_dim.1, state.color());
            }
        }
    }

    fn recalculate_dim(&mut self, screen: (i32, i32)) {
        let dim_diff = ((screen.0 as f64 * 0.15) as i32, (screen.1 as f64 * 0.05) as i32);
        self.dim = (dim_diff.0, dim_diff.1, screen.0 - dim_diff.0*2, screen.1 - dim_diff.1*2);
        self.cell_dim = ((self.dim.2 as f64 / self.size.0 as f64) as i32, (self.dim.3 as f64 / self.size.1 as f64) as i32);
        println!("{:?}", self.cell_dim);
    }

    fn get_cell(&self, x: i32, y: i32) -> Option<&Cell> {
        if x >= 0 && x < self.size.0 && y >= 0 && y < self.size.1 {
            return Some(&self.cells[x as usize][y as usize]);
        }
        None
    }
}

#[derive(Debug, Clone, Copy)]
struct Cell {
    pub state: CellStates
}

impl Cell {
    fn new() -> Cell {
        Cell { state: CellStates::Empty }
    }
}

#[derive(Debug, Clone, Copy)]
enum CellStates {
    Empty,
    Sand,
}

impl CellStates {
    pub fn color(&self) -> Color {
        match &self {
            Self::Empty => Color::BLUE,
            Self::Sand => Color::YELLOW,
        }
    }
}