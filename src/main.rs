/*
* Conway's Game of Life
*
* TODO: maybe paint after rules have been applied to whole table
* TODO: implement wraping around canvas
* TODO: figured out a way to zoom in/out of canvas for smaller patterns
* TODO: update Cells struct to work with dynamic dimensions
*/

// use std::{time, thread, cell};
use pixel_canvas::{Canvas, Color, input::{MouseState, glutin::event::{VirtualKeyCode, ElementState}}, prelude::*};
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

fn main() {
    // Configure the window that you want to draw in. You can add an event
    // handler to build interactive art. Input handlers for common use are
    // provided.
    let canvas = Canvas::new(512, 512)
        .title("Life")
        .state(KeyboardState::new())
        .input(KeyboardState::handle_input);

    let mut cells = Cells {
        states: [[false; 512]; 512],
    };

    cells.randomize();

    // cells.mosaic();

    // ###
    // cells.states[250][250] = true;
    // cells.states[250][251] = true;
    // cells.states[250][252] = true;

    // ##
    // #
    // cells.states[300][300] = true;
    // cells.states[300][299] = true;
    // cells.states[299][299] = true;

    //  #
    // ###
    // cells.states[200][200] = true;
    // cells.states[199][200] = true;
    // cells.states[199][199] = true;
    // cells.states[199][201] = true;

    // ##
    // ##
    // cells.states[150][150] = true;
    // cells.states[150][149] = true;
    // cells.states[149][150] = true;
    // cells.states[149][149] = true;

    // glider triangles
    // cells.glider(50, 50);
    // cells.glider(50, 45);
    // cells.glider(55, 47);

    // cells.glider(60, 60);
    // cells.glider(60, 55);
    // cells.glider(65, 57);

    // cells.glider(50, 50);
    // cells.glider(50, 45);
    // cells.glider(55, 47);
    // =======================

    canvas.render(move |keyboard, image| {
        // color to paint cell with
        let mut cell_color: Color;
        // counter for living neighbor to apply game rules
        let mut living_neighbors: u8;
        // Modify the `image` based on your state.
        let width = image.width() as usize;
        // we need this to avoid updating the same array we are iterating
        let states_lookup = cells.states.clone();

        if keyboard.virtual_key_code == VirtualKeyCode::R && keyboard.state == ElementState::Pressed {
            cells.randomize();
        }
    

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

                // count living neighbors
                living_neighbors = 0;
                for (ny, nx) in cells.neighbor_indexes(x, y) {
                    if states_lookup[ny][nx] {
                        living_neighbors += 1;
                    }
                }

                // alive
                if states_lookup[y][x] {
                    // stays alive if it has either 2 or 3 live neighbors
                    if living_neighbors != 2 && living_neighbors != 3 {
                        // coocentric expansion in all directions
                        // if living_neighbors < 2 {
                        cells.states[y][x] = false;
                    }
                // dead
                } else {
                    // springs to life only in the case that it has 3 live neighbors
                    if living_neighbors == 3 {
                        // coocentric expansion in all directions
                        // if living_neighbors == 2 {
                        cells.states[y][x] = true;
                    }
                }

                if cells.states[y][x] {
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
