/*
* Conway's Game of Life
*
* TODO: implement wraping around canvas
* TODO: figured out a way to zoom in/out of canvas for smaller patterns
* TODO: update Cells struct to work with dynamic dimensions
*/

use std::{fs, process::exit};

// use std::{time, thread, cell};
use pixel_canvas::{input::glutin::event::VirtualKeyCode, Canvas, Color};
use rand::{self};

mod keyboard;
pub use crate::keyboard::KeyboardState;

#[derive(Clone, PartialEq, Copy)]
enum State {
    Alive,
    Dead,
}

struct Cells {
    states: [[State; 512]; 512],
}

impl Cells {
    fn new() -> Self {
        Self {
            states: [[State::Dead; 512]; 512],
        }
    }

    fn randomize(&mut self) {
        for y in 1..511 {
            for x in 1..511 {
                if rand::random() {
                    self.states[y][x] = State::Alive;
                }
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
    states_lookup: [[State; 512]; 512],
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
        let mut living_neighbors = 0;
        for (ny, nx) in self.cells.neighbor_indexes(x, y) {
            if self.states_lookup[ny][nx] == State::Alive {
                living_neighbors += 1;
            }
        }
        if self.states_lookup[y][x] == State::Alive {
            // stays alive if it has either 2 or 3 live neighbors
            if living_neighbors != 2 && living_neighbors != 3 {

                self.cells.states[y][x] = State::Dead;
            }
        } else {
            // springs to life only in the case that it has 3 live neighbors
            if living_neighbors == 3 {
                self.cells.states[y][x] = State::Alive;
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

        // self.cells = Cells::new();
        
        for (y, l) in pattern.iter().enumerate() {
            for (x, c) in l.iter().enumerate() {
                if *c != '.' {
                    self.cells.states[y + pos_y][x + pos_x] = State::Alive;
                } else {
                    self.cells.states[y + pos_y][x + pos_x] = State::Dead;
                }
            }
        }
    }
}

fn main() {
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
            Some(VirtualKeyCode::Key2) => {
                // puffer array
                for i in (12..490).step_by(35) {
                    life.load_pattern("patterns/frothing_puffer.txt", i, 400);
                }
            }
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

                if life.cells.states[y][x] == State::Alive {
                    cell_color = Color { r: 150, g: 150, b: 150 };
                } else {
                    cell_color = Color { r: 0, g: 0, b: 0 };
                }

                *pixel = cell_color;
            }
        }
    });
}
