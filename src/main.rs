/*
* Conway's Game of Life
*
* TODO: maybe paint after rules have been applied to whole table
* TODO: implement wraping around canvas
* TODO: figured out a way to zoom in/out of canvas for smaller patterns
* TODO: update Cells struct to work with dynamic dimensions
*/

use std::{process::exit, fs};

// use std::{time, thread, cell};
use pixel_canvas::{
    input::glutin::event::{ElementState, VirtualKeyCode},
    Canvas, Color,
};
use rand::{self, Rng};

mod keyboard;
pub use crate::keyboard::KeyboardState;

struct Cells {
    // table with the state of every single cell:
    // alive = true
    // dead = false
    states: [[bool; 512]; 512],
}

impl Cells {
    fn new() -> Self {
        Self {
            states: [[false; 512]; 512],
        }
    }

    fn glider(&mut self, x: usize, y: usize) {
        // ###
        // #
        //  #
        self.states[y][x] = true;
        self.states[y][x - 1] = true;
        self.states[y][x - 2] = true;
        self.states[y - 1][x - 2] = true;
        self.states[y - 2][x - 1] = true;
    }

    fn mosaic(&mut self) {
        self.states[255][255] = true;
        self.states[255][256] = true;
        self.states[255][257] = true;
        //
        self.states[10][10] = true;
        self.states[10][11] = true;
        self.states[10][12] = true;
        //
        self.states[10][500] = true;
        self.states[10][501] = true;
        self.states[10][502] = true;
        //
        self.states[500][10] = true;
        self.states[500][11] = true;
        self.states[500][12] = true;
        //
        self.states[500][500] = true;
        self.states[500][501] = true;
        self.states[500][502] = true;
    }

    fn randomize(&mut self) {
        for y in 1..511 {
            for x in 1..511 {
                self.states[y][x] = rand::random();
            }
        }
    }

    fn neighbor_indexes(&self, cell_x: usize, cell_y: usize) -> [(usize, usize); 8] {
        [
            (cell_y - 1, cell_x),     // bellow
            (cell_y + 1, cell_x),     // above
            (cell_y, cell_x + 1),     // right
            (cell_y, cell_x - 1),     // left
            (cell_y + 1, cell_x - 1), // left upper corner
            (cell_y + 1, cell_x + 1), // right upper corner
            (cell_y - 1, cell_x - 1), // left lower corner
            (cell_y - 1, cell_x + 1), // right lower corner
        ]
    }
}

struct Life {
    cells: Cells,
    states_lookup: [[bool; 512]; 512],
}

impl Life {
    fn new() -> Self {
        let cells = Cells::new();
        Self {
            cells: Cells::new(),
            states_lookup: cells.states.clone(),
        }
    }

    fn update_lookup(&mut self) {
        self.states_lookup = self.cells.states.clone();
    }

    fn apply_rules(&mut self, x: usize, y: usize) {
        // count living neighbors
        let mut living_neighbors = 0;
        for (ny, nx) in self.cells.neighbor_indexes(x, y) {
            if self.states_lookup[ny][nx] {
                living_neighbors += 1;
            }
        }

        // alive
        if self.states_lookup[y][x] {
            // stays alive if it has either 2 or 3 live neighbors
            if living_neighbors != 2 && living_neighbors != 3 {
                // coocentric expansion in all directions
                // if living_neighbors < 2 {
                self.cells.states[y][x] = false;
            }
        // dead
        } else {
            // springs to life only in the case that it has 3 live neighbors
            if living_neighbors == 3 {
                // coocentric expansion in all directions
                // if living_neighbors == 2 {
                self.cells.states[y][x] = true;
            }
        }
    }

    fn load_pattern(&mut self, path: &str, pos_x: usize, pos_y: usize) {
        let content = fs::read_to_string(path).expect("Oops!");

        let pattern: Vec<Vec<char>> = content
        .lines()
        .skip_while(|l| l.starts_with('!'))
        .map(|l| l.chars().collect())
        .collect();

        self.cells = Cells::new();

        for (y, l) in pattern.iter().enumerate() {
            for (x, c) in l.iter().enumerate() {
                self.cells.states[y + pos_y][x + pos_x] = *c != '.';
            }
        }
    }
}

fn main() {
    // Configure the window that you want to draw in. You can add an event
    // handler to build interactive art. Input handlers for common use are
    // provided.
    let canvas = Canvas::new(512, 512)
        .title("Life")
        .state(KeyboardState::new())
        .input(KeyboardState::handle_input);

    let mut life = Life::new();

    canvas.render(move |keyboard, image| {
        // color to paint cell with
        let mut cell_color: Color;
        // Modify the `image` based on your state.
        let width = image.width() as usize;

        // handle keyboard commands
        match keyboard.key_pressed() {
            Some(VirtualKeyCode::Escape) => exit(0),
            Some(VirtualKeyCode::Key1) => life.cells.randomize(),
            Some(VirtualKeyCode::Key2) => life.load_pattern("patterns/gosper_glider_gun.txt", 255, 255),
            Some(VirtualKeyCode::Key3) => life.load_pattern("patterns/oscilator.txt", 255, 255),
            _ => (),
        }

        // we need this to avoid updating the same array we are iterating
        life.update_lookup();

        for (y, row) in image.chunks_mut(width).enumerate() {
            // skip corners for now
            if y == 0 || y == width - 1 {
                continue;
            }

            for (x, pixel) in row.iter_mut().enumerate() {
                // skip corners for now
                if x == 0 || x == width - 1 {
                    continue;
                }

                life.apply_rules(x, y);

                if life.cells.states[y][x] {
                    // println!("{},{},{}", (*pixel).r, (*pixel).g, (*pixel).b);

                    // let mut red  = ((*pixel).r + 1)  % 255;
                    // let mut green  = ((*pixel).g + 2) % 254;
                    // let mut blue = ((*pixel).b + 1) % 255;

                    // if red == 0 && green == 0 && blue == 0 {
                    //     red = rand::thread_rng().gen_range(0..50);
                    //     green = rand::thread_rng().gen_range(0..252);
                    //     blue = rand::thread_rng().gen_range(0..50);
                    // }
                    cell_color = Color {
                        // r: rand::thread_rng().gen_range(0..255),
                        // g: rand::thread_rng().gen_range(0..255),
                        // b: rand::thread_rng().gen_range(0..255),
                        // r: red,
                        // g: green,
                        // b: blue,
                        r: 150,
                        g: 150,
                        b: 150,
                    };
                } else {
                    cell_color = Color { r: 0, g: 0, b: 0 };
                }

                *pixel = cell_color;
            }
        }
    });
}
