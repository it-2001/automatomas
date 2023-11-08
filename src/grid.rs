use raylib::prelude::*;
use rand::prelude::*;

use crate::*;

#[derive(Clone, Debug)]
pub struct Grid {
    pub cells: Vec<Vec<Cell>>,
    pub rng: ThreadRng,
    pub size: (i32, i32),
    pub dim: (f64, f64, f64, f64),
    pub cell_dim: (f64, f64),
}

impl Grid {
    pub fn new((width, height): (i32, i32)) -> Grid {
        let mut cells = Vec::with_capacity(width as usize);
        for _ in 0..width {
            cells.push(vec![Cell::new(); height as usize]);
        }
        let rng = rand::thread_rng();

        Grid {
            cells,
            rng,
            size: (width, height),
            dim: (0.,0.,0.,0.),
            cell_dim: (0., 0.),
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        const COLOR: CellStates = CellStates::Air;
        d.draw_rectangle(self.dim.0 as i32, self.dim.1 as i32, self.dim.2 as i32, self.dim.3 as i32, CellStates::color(&COLOR));
        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
                let state = self.cells[x as usize][y as usize].state;
                let rect = (self.dim.0 + x as f64 * self.cell_dim.0, self.dim.1 + y as f64 * self.cell_dim.1, self.cell_dim.0, self.cell_dim.1);
                d.draw_rectangle(rect.0 as i32, rect.1 as i32, rect.2.ceil() as i32, rect.3.ceil() as i32, state.color());
            }
        }
    }

    pub fn recalculate_dim(&mut self, screen: (i32, i32)) {
        let dim_diff = ((screen.0 as f64 * 0.15), (screen.1 as f64 * 0.05));
        self.dim = (dim_diff.0, dim_diff.1, screen.0 as f64 - dim_diff.0*2., screen.1 as f64 - dim_diff.1*2.);
        self.cell_dim = ((self.dim.2 as f64 / self.size.0 as f64), (self.dim.3 as f64 / self.size.1 as f64));
    }

    pub fn get_cell(&self, x: i32, y: i32) -> Option<&Cell> {
        if x >= 0 && x < self.size.0 && y >= 0 && y < self.size.1 {
            return Some(&self.cells[x as usize][y as usize]);
        }
        None
    }

    pub fn step(&mut self) {
        let cell = (self.rng.gen_range(0..self.size.0), self.rng.gen_range(0..self.size.1));
        let state = self.cells[cell.0 as usize][cell.1 as usize].state;
        match state {
            CellStates::Air => (),
            CellStates::Sand => {
                match self.get_cell(cell.0, cell.1 + 1) {
                    Some(other) => {
                        if other.state.hardness() > state.hardness() {
                            self.swap(cell.0, cell.1, (cell.0, cell.1 + 1));
                            return;
                        }
                    }
                    None => ()
                }
                let mut side = self.rng.gen_range(0..2) * 2 - 1;
                for _ in 0..2 {
                    match self.get_cell(cell.0 + side, cell.1 + 1) {
                        Some(other) => {
                            if other.state.hardness() > state.hardness() {
                                self.swap(cell.0, cell.1, (cell.0 + side, cell.1 + 1));
                                return;
                            }
                        },
                        None => (),
                    }
                    side *= -1;
                }
            },
            CellStates::Water => {
                match self.get_cell(cell.0, cell.1 + 1) {
                    Some(other) => {
                        if other.state.hardness() > state.hardness() {
                            self.swap(cell.0, cell.1, (cell.0, cell.1 + 1));
                            return;
                        }
                    }
                    None => ()
                }
                let mut side = self.rng.gen_range(0..2) * 2 - 1;
                for _ in 0..2 {
                    match self.get_cell(cell.0 + side, cell.1 + 1) {
                        Some(other) => {
                            if other.state.hardness() > state.hardness() {
                                self.swap(cell.0, cell.1, (cell.0 + side, cell.1 + 1));
                                return;
                            }
                        },
                        None => (),
                    }
                    side *= -1;
                }
                for _ in 0..2 {
                    match self.get_cell(cell.0 + side, cell.1) {
                        Some(other) => {
                            if other.state.hardness() > state.hardness() {
                                self.swap(cell.0, cell.1, (cell.0 + side, cell.1));
                                return;
                            }
                        },
                        None => (),
                    }
                    side *= -1;
                }
            }
            CellStates::Plague => {
                let victim = (self.rng.gen_range(0..2) * 2 - 1 + cell.0, self.rng.gen_range(0..2) * 2 - 1  + cell.1);
                match self.get_cell(victim.0, victim.1) {
                    Some(other) => {
                        if other.state.hardness() >= state.hardness() {
                            self.set(victim.0, victim.1, state);
                            return;
                        }
                    },
                    None => (),
                }
            },
            CellStates::Wall => (),
            CellStates::Barrier => (),
            CellStates::Border => unreachable!("Border should not be stepped"),
        }
    }

    pub fn set(&mut self, x: i32, y: i32, state: CellStates) {
        if !self.bounds(x, y) {
            return;
        }
        self.cells[x as usize][y as usize].state = state;
    }

    pub fn swap(&mut self, x: i32, y: i32, other: (i32, i32)) {
        if !self.bounds(x, y) || !self.bounds(other.0, other.1) {
            return;
        }
        let state = self.cells[x as usize][y as usize].state;
        self.cells[x as usize][y as usize].state = self.cells[other.0 as usize][other.1 as usize].state;
        self.cells[other.0 as usize][other.1 as usize].state = state;
    }

    pub fn bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.size.0 && y >= 0 && y < self.size.1
    }
}