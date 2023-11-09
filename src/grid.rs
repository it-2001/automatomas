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
            CellStates::Fire(level) => {
                for gp in self.find_all_around(cell.0, cell.1, &CellStates::Gunpowder) {
                    self.cell_unchecked(gp.0, gp.1).state = CellStates::Spark
                }
                for gp in self.find_all_around(cell.0, cell.1, &CellStates::Water) {
                    self.cell_unchecked(gp.0, gp.1).state = CellStates::Vapor
                }
                let rand = self.rng.gen_range(-1..2);
                match self.get_cell(cell.0 + rand, cell.1 - 1) {
                    Some(other) => {
                        if other.state.hardness() > state.hardness() && self.rng.gen_range(0..50) > 20 {
                            self.set(cell.0 + rand, cell.1 - 1, state)
                        }
                    }
                    None => {

                    }
                }
                match self.rng.gen_range(0..50) {
                    0..=5 => {
                        self.cell_unchecked(cell.0, cell.1).state = CellStates::Air
                    }
                    0..=40 => {
                        if level == 0 {
                            self.cell_unchecked(cell.0, cell.1).state = CellStates::Air
                        }else {
                            self.cell_unchecked(cell.0, cell.1).state = CellStates::Fire(level - 1)
                        }
                    }
                    _ => {}
                }
            }
            CellStates::Gunpowder => {
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
            }
            CellStates::Spark => {
                let direction = loop {
                    let direction = (self.rng.gen_range(-1..2), self.rng.gen_range(-1..2));
                    if direction != (0, 0) {
                        break direction;
                    }
                };
                let mut power = self.rng.gen_range(0..45);
                while power > 0 {
                    self.set(cell.0 + direction.0 * power, cell.1 + direction.1 * power, CellStates::Fire(2));
                    power -= 1;
                }
                self.set(cell.0, cell.1, CellStates::Air)
            }
            CellStates::Vapor => {
                match self.rng.gen_range(0..400) {
                    0..=1 => {
                        self.cell_unchecked(cell.0, cell.1).state = CellStates::Air
                    }
                    0..=3 => {
                        self.cell_unchecked(cell.0, cell.1).state = CellStates::Water
                    }
                    _ => {}
                }
                let rand = self.rng.gen_range(-1..2);
                match self.get_cell(cell.0 + rand, cell.1 - 1) {
                    Some(other) => {
                        if other.state.hardness() > state.hardness() && self.rng.gen_range(0..50) > 20 {
                            self.swap(cell.0, cell.1, (cell.0 + rand, cell.1 - 1))
                        }
                    }
                    None => {

                    }
                }

            }
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

    pub fn cell_unchecked(&mut self, x: i32, y: i32) -> &mut Cell {
        &mut self.cells[x as usize][y as usize]
    }

    pub fn is_around(&self, x: i32, y: i32, state: &CellStates) -> bool {
        for i in -1..2 {
            for j in -1..2 {
                if i == 0 && j == 0 {
                    continue;
                }
                match self.get_cell(x + i, y + j) {
                    Some(cell) => {
                        if cell.state == *state {
                            return true
                        }
                    }
                    None => ()
                }
            }
        }
        false
    }

    pub fn find_all_around(&self, x: i32, y: i32, state: &CellStates) -> Vec<(i32, i32)> {
        let mut result = Vec::new();
        for i in -1..2 {
            for j in -1..2 {
                if i == 0 && j == 0 {
                    continue;
                }
                match self.get_cell(x + i, y + j) {
                    Some(cell) => {
                        if cell.state == *state {
                            result.push((x+i, y+j));
                        }
                    }
                    None => ()
                }
            }
        }
        result
    }
}