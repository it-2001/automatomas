use std::collections::HashMap;

use raylib::prelude::*;

use crate::grid::*;

use cstr::cstr;

pub trait Window {
    fn draw(&mut self, grid: &mut Grid, d: &mut RaylibDrawHandle) -> bool;
}

pub struct About {

}

impl Window for About {
    fn draw(&mut self, grid: &mut Grid, d: &mut RaylibDrawHandle) -> bool {
        let result = d.gui_window_box(Rectangle::new(0.0, 0.0, 200.0, 200.0), Some(cstr!("About")));

        !result
    }
}

pub struct Help {

}

impl Window for Help {
    fn draw(&mut self, grid: &mut Grid, d: &mut RaylibDrawHandle) -> bool {
        let result = d.gui_window_box(Rectangle::new(0.0, 0.0, 200.0, 200.0), Some(cstr!("Help")));

        !result
    }
}

pub fn get_all_windows() -> HashMap<String, Box<dyn Window>> {
    let mut windows: HashMap<String, Box<dyn Window>> = HashMap::new();

    windows.insert("About".to_string(), Box::new(About{}));
    windows.insert("Help".to_string(), Box::new(Help{}));

    windows
}