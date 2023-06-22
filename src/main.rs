/*
* Conway's Game of Life
*
* TODO: maybe paint after rules have been applied to whole table
* TODO: implement wraping around canvas
* TODO: figured out a way to zoom in/out of canvas for smaller patterns
*
*/

// use std::{time, thread, cell};
use rand::{self, Rng};
use pixel_canvas::{Canvas, Color};

fn glider(cells_state: &mut[[bool; 512]; 512], pos: usize) {
    // ###
    // #
    //  #
    cells_state[pos][pos] = true;
    cells_state[pos][pos-1] = true;
    cells_state[pos][pos-2] = true;
    cells_state[pos-1][pos-2] = true;
    cells_state[pos-2][pos-1] = true; 
}

fn main() {
    // Configure the window that you want to draw in. You can add an event
    // handler to build interactive art. Input handlers for common use are
    // provided.
    let canvas: Canvas<()> = Canvas::new(512, 512)
        .title("Life")
        .show_ms(true);

    // table with the state of every single cell:
    // alive = true
    // dead = false
    let mut cells_state: [[bool; 512]; 512] = [[false; 512]; 512];

    // ###
    // cells_state[100][100] = true;
    // cells_state[100][101] = true;
    // cells_state[100][102] = true;

    cells_state[15][15] = true;
    cells_state[15][16] = true;
    cells_state[15][17] = true;

    cells_state[15][500] = true;
    cells_state[15][501] = true;
    cells_state[15][502] = true;

    cells_state[500][15] = true;
    cells_state[500][16] = true;
    cells_state[500][17] = true;

    cells_state[500][500] = true;
    cells_state[500][501] = true;
    cells_state[500][502] = true;

    // cells_state[15][15] = true;
    // cells_state[15][16] = true;
    // cells_state[15][17] = true;

    // cells_state[15][15] = true;
    // cells_state[15][16] = true;
    // cells_state[15][17] = true;

    // ##
    // #
    // cells_state[300][300] = true;
    // cells_state[300][299] = true;
    // cells_state[299][299] = true;

    //  #
    // ###
    // cells_state[200][200] = true;
    // cells_state[199][200] = true;
    // cells_state[199][199] = true;
    // cells_state[199][201] = true;

    // ##
    // ##
    // cells_state[150][150] = true;
    // cells_state[150][149] = true;
    // cells_state[149][150] = true;
    // cells_state[149][149] = true;
    
    // for i in (60..75).step_by(5) {
    //     glider(&mut cells_state, i);
    // }

    // for y in 1..511 {
    //     for x in 1..511 {
    //         cells_state[y][x] = rand::random();
    //     }
    // }

    // The canvas will render for you at up to 60fps.
    canvas.render(move |_mouse, image| {

        // Modify the `image` based on your state.
        let width = image.width() as usize;
        // counter for living neighbor to apply game rules
        let mut living_neighbors: u32;
        // used to iterate through the neighbor cells
        let mut neighbor_indexes: [(usize, usize); 8];
        // we need this to avoid updating the same array we are iterating
        let cells_state_lookup = cells_state.clone();

        // iterate all pixels and apply rules
        for (y, row) in image.chunks_mut(width).enumerate() {

            for (x, pixel) in row.iter_mut().enumerate() {
                living_neighbors = 0;

                // skip corners for now
                if x == 0 || x == width - 1 || y == 0 || y == width - 1 {
                    continue;
                }

                neighbor_indexes = [
                    (y-1, x),   // bellow
                    (y+1, x),   // above
                    (y, x+1),   // right
                    (y, x-1),   // left
                    (y+1, x-1), // left upper corner
                    (y+1, x+1), // right upper corner
                    (y-1, x-1), // left lower corner
                    (y-1, x+1), // right lower corner
                ];

                // count living neighbor cells
                for (ny, nx) in neighbor_indexes {
                    if cells_state_lookup[ny][nx] {
                        living_neighbors += 1;
                    }
                }

                // alive
                if cells_state_lookup[y][x] {
                    // stays alive if it has either 2 or 3 live neighbors 
                    // if living_neighbors != 2 && living_neighbors != 3 {
                    if living_neighbors < 2 {
                        cells_state[y][x] = false;
                    }
                    // dead
                } else {
                    // springs to life only in the case that it has 3 live neighbors
                    if living_neighbors == 2 {
                        cells_state[y][x] = true;
                    }
                }

                // paint cell white if it's alive
                if cells_state[y][x] {
                    // println!("{},{},{}", (*pixel).r, (*pixel).g, (*pixel).b);

                    let red  = ((*pixel).r + 1) % 255;
                    let green  = ((*pixel).g + 2) % 254;
                    let blue = ((*pixel).b + 1) % 255;

                    *pixel = Color {
                        // r: rand::thread_rng().gen_range(0..255),
                        // g: rand::thread_rng().gen_range(0..255),
                        // b: rand::thread_rng().gen_range(0..255),
                        r: red,
                        g: green,
                        b: blue,
                    }
                } else {
                    *pixel = Color {
                        r: 0,
                        g: 0,
                        b: 0,
                    }
                }
            }
        }

    });
}