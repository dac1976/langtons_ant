// This file is part of CoreLibrary containing useful reusable utility
// classes.
//
// Copyright (C) 2020 onwards, Duncan Crutchley
// Contact <dac1976github@outlook.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License and GNU Lesser General Public License
// for more details.
//
// You should have received a copy of the GNU General Public License
// and GNU Lesser General Public License along with this program. If
// not, see <http://www.gnu.org/licenses/>.

extern crate float_cmp;
extern crate piston_window;
extern crate rand;

use float_cmp::*;
use piston_window::*;
use rand::prelude::*;
use std::io;
use std::process;

//-----------------------------------------------------------------------------
// ENUMS, STRUCTS AND IMPLS
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// Structure to hold colour information.
struct Colour {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Colour {
    // Compare this Colour to another Colour instance.
    fn compare(&self, other: &Colour) -> bool {
        approx_eq!(f32, self.r, other.r)
            && approx_eq!(f32, self.g, other.g)
            && approx_eq!(f32, self.b, other.b)
            && approx_eq!(f32, self.a, other.a)
    }

    fn to_rgba(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

//-----------------------------------------------------------------------------
// Direction to move.
#[derive(Copy, Clone)]
enum Direction {
    L,
    R,
}

//-----------------------------------------------------------------------------
// Facing enum for encoding way ant is pointing.
enum Facing {
    N,
    E,
    S,
    W,
}

//-----------------------------------------------------------------------------
// The Ant structure defining its position, movement rule, associated colours
// and iteration count.
struct Ant {
    pos_x: usize,
    pos_y: usize,
    rule: Vec<Direction>,
    colours: Vec<Colour>,
    facing: Facing,
    stalled: bool,
    iterations: u64,
}

impl Ant {
    fn new(x: usize, y: usize) -> Ant {
        Ant {
            pos_x: x,
            pos_y: y,
            rule: Vec::new(),
            colours: Vec::new(),
            facing: Facing::N,
            stalled: false,
            iterations: 0,
        }
    }
}

//-----------------------------------------------------------------------------
// The row structure defnies the current colour code for each cell 
// on a given row.
struct Row {
    cells: Vec<usize>,
}

impl Row {
    fn new(num_cells: usize, clr_idx: usize) -> Row {
        let mut r = Row { cells: Vec::new() };
        r.cells.resize(num_cells, clr_idx);
        r
    }
}

//-----------------------------------------------------------------------------
// The grid structure encoding the state of each cell as a numerical value
// between 0 and n - 1, where there are n colours, one for each move in
// a rule.
struct Grid {
    rows: Vec<Row>,
}

impl Grid {
    fn new(num_rows: usize, num_cols: usize, clr_idx: usize) -> Grid {
        let mut g = Grid {
            rows: Vec::with_capacity(num_rows),
        };
        while g.rows.len() != num_rows {
            g.rows.push(Row::new(num_cols, clr_idx));
        }
        g
    }
}

//-----------------------------------------------------------------------------
// FUNCTIONS
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// Function to generate a random colour.
fn generate_random_colour() -> [f32; 4] {
    let mut rng = rand::thread_rng();
    let r: f32 = rng.gen(); // generates a float between 0 and 1
    let g: f32 = rng.gen(); // generates a float between 0 and 1
    let b: f32 = rng.gen(); // generates a float between 0 and 1
    let rgba = [r, g, b, 1.0]; // [Red, Green, Blue, Alpha]
    rgba
}

//-----------------------------------------------------------------------------
// Function to create a colour object and randomly fill its values.
fn create_random_colour() -> Colour {
    let c = generate_random_colour();
    let colour: Colour = Colour {
        r: c[0],
        g: c[1],
        b: c[2],
        a: c[3],
    };
    colour
}

//-----------------------------------------------------------------------------
// Print out a title greeting message to the user in the console.
fn print_title() {
    println!("***************************");
    println!("* LANGTON's ANT SIMULATOR *");
    println!("***************************");
    println!("");
}

//-----------------------------------------------------------------------------
// Print our requests to the user for control parameters.
fn print_input_requests() -> (String, i32, u32, f64) {
    println!("Please enter a rule using L and R characters, e.g. LR or RLLR etc. Press enter to use default \"RL\". > ");

    let mut rule = String::new();

    io::stdin()
        .read_line(&mut rule)
        .expect("Failed to read input");

    let mut rule = rule.trim().to_string();

    if rule.is_empty() || (rule == "\r\n") || (rule == "\r") || (rule == "\n") {
        rule = String::from("RL");
    }

    for c in rule.chars() {
        if (c != 'L') && (c != 'R') {
            println!("ERROR - Invalid rule input: {}", rule);
            process::exit(0);
        }
    }

    println!(
        "Please enter number of moves per second (1, 2, 5, 10, 20, 50, 100, 200, 500, 1000). Press enter to use default 10. > "
    );

    let mut mps = String::new();

    io::stdin()
        .read_line(&mut mps)
        .expect("Failed to read input");

    if mps.is_empty() || (mps == "\r\n") || (mps == "\r") || (mps == "\n") {
        mps = String::from("10");
    }

    let mps: i32 = match mps.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("ERROR - Invalid moves per second = {}", mps);
            process::exit(0);
        }
    };

    let check_mps: Vec<i32> = vec![1, 2, 5, 10, 20, 50, 100, 200, 500, 1000];

    if !check_mps.contains(&mps) {
        println!("ERROR - Invalid moves per second = {}", mps);
        process::exit(0);
    }

    println!("Please enter a grid size as a number of squares (10 - 1000). Press enter to use default 150 squares. > ");

    let mut grid_size = String::new();

    io::stdin()
        .read_line(&mut grid_size)
        .expect("Failed to read input");

    if grid_size.is_empty() || (grid_size == "\r\n") || (grid_size == "\r") || (grid_size == "\n") {
        grid_size = String::from("150");
    }

    let grid_size: u32 = match grid_size.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("ERROR - Invalid grid size = {}", grid_size);
            process::exit(0);
        }
    };

    if (grid_size < 10) || (grid_size > 1000) {
        println!("ERROR - Invalid grid size = {}", grid_size);
        process::exit(0);
    }

    println!("Please enter the size of a grid square as a number of pixels (1 - 20). Press enter to use default 5 pixels. > ");

    let mut square_size = String::new();

    io::stdin()
        .read_line(&mut square_size)
        .expect("Failed to read input");

    if square_size.is_empty()
        || (square_size == "\r\n")
        || (square_size == "\r")
        || (square_size == "\n")
    {
        square_size = String::from("5");
    }

    let square_size: f64 = match square_size.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("ERROR - Invalid grid square size = {}", grid_size);
            process::exit(0);
        }
    };

    if (square_size < 1.0) || (square_size > 20.0) {
        println!("ERROR - Invalid grid square size = {}", grid_size);
        process::exit(0);
    }

    let dim: u32 = grid_size * (square_size as u32);

    if dim > 1000 {
        println!(
            "ERROR - Invalid grid dimension, grid_size * square_size must be <= 1000, dim = {}",
            dim
        );
        process::exit(0);
    }

    (rule.to_string(), mps, grid_size, square_size)
}

//-----------------------------------------------------------------------------
// Move ant coming from originally facing North.
fn move_from_north(ant_dir: Direction, dim: usize, ant: &mut Ant) {
    match ant_dir {
        Direction::L => {
            // Set new direction to face.
            ant.facing = Facing::W;

            // Move ant in correct direction. Checking for
            // hitting boundary, in which case we mark ant
            // as stalled.
            if 0 == ant.pos_x {
                ant.stalled = true;
            } else {
                ant.pos_x -= 1;
            }
        }
        Direction::R => {
            // Set new direction to face.
            ant.facing = Facing::E;

            // Move ant in correct direction. Checking for
            // hitting boundary, in which case we mark ant
            // as stalled.
            if dim - 1 == ant.pos_x {
                ant.stalled = true;
            } else {
                ant.pos_x += 1;
            }
        }
    }
}

//-----------------------------------------------------------------------------
// Move ant coming from originally facing East.
fn move_from_east(ant_dir: Direction, dim: usize, ant: &mut Ant) {
    match ant_dir {
        Direction::L => {
            // Set new direction to face.
            ant.facing = Facing::N;

            // Move ant in correct direction. Checking for
            // hitting boundary, in which case we mark ant
            // as stalled.
            if 0 == ant.pos_y {
                ant.stalled = true;
            } else {
                ant.pos_y -= 1;
            }
        }
        Direction::R => {
            // Set new direction to face.
            ant.facing = Facing::S;

            // Move ant in correct direction. Checking for
            // hitting boundary, in which case we mark ant
            // as stalled.
            if dim - 1 == ant.pos_y {
                ant.stalled = true;
            } else {
                ant.pos_y += 1;
            }
        }
    }
}

//-----------------------------------------------------------------------------
// Move ant coming from originally facing South.
fn move_from_south(ant_dir: Direction, dim: usize, ant: &mut Ant) {
    match ant_dir {
        Direction::L => {
            // Set new direction to face.
            ant.facing = Facing::E;

            // Move ant in correct direction. Checking for
            // hitting boundary, in which case we mark ant
            // as stalled.
            if dim - 1 == ant.pos_x {
                ant.stalled = true;
            } else {
                ant.pos_x += 1;
            }
        }
        Direction::R => {
            // Set new direction to face.
            ant.facing = Facing::W;

            // Move ant in correct direction. Checking for
            // hitting boundary, in which case we mark ant
            // as stalled.
            if 0 == ant.pos_x {
                ant.stalled = true;
            } else {
                ant.pos_x -= 1;
            }
        }
    }
}

//-----------------------------------------------------------------------------
// Move ant coming from originally facing West.
fn move_from_west(ant_dir: Direction, dim: usize, ant: &mut Ant) {
    match ant_dir {
        Direction::L => {
            // Set new direction to face.
            ant.facing = Facing::S;

            // Move ant in correct direction. Checking for
            // hitting boundary, in which case we mark ant
            // as stalled.
            if dim - 1 == ant.pos_y {
                ant.stalled = true;
            } else {
                ant.pos_y += 1;
            }
        }
        Direction::R => {
            // Set new direction to face.
            ant.facing = Facing::N;

            // Move ant in correct direction. Checking for
            // hitting boundary, in which case we mark ant
            // as stalled.
            if 0 == ant.pos_y {
                ant.stalled = true;
            } else {
                ant.pos_y -= 1;
            }
        }
    }
}

//-----------------------------------------------------------------------------
// Compute new position of ant updating grif colours as we move ant.
fn compute_ant_position(ant: &mut Ant, grid: &mut Grid) {
    // Has ant stalled?
    if ant.stalled {
        return;
    }

    // Grab the current colour index for the ant's current position.
    let mut cell_clr_idx = grid.rows[ant.pos_y].cells[ant.pos_x];

    if usize::max_value() == cell_clr_idx {
        cell_clr_idx = 0;
    }

    // Grab direction we need to turn.
    let ant_dir = ant.rule[cell_clr_idx];

    // Increment cell colour index.
    cell_clr_idx += 1;

    if ant.colours.len() == cell_clr_idx {
        cell_clr_idx = 0;
    }

    grid.rows[ant.pos_y].cells[ant.pos_x] = cell_clr_idx;

    // Grab the grid dimension.
    let dim = grid.rows.len();

    // Move ant in correctdirection based on way it is currently facing.
    match ant.facing {
        Facing::N => move_from_north(ant_dir, dim, ant),
        Facing::E => move_from_east(ant_dir, dim, ant),
        Facing::S => move_from_south(ant_dir, dim, ant),
        Facing::W => move_from_west(ant_dir, dim, ant),
    }

    // Increment the iteration count.
    if u64::max_value() == ant.iterations {
        ant.stalled = true;
    } else {
        ant.iterations += 1;
    }
}

//-----------------------------------------------------------------------------
// The applications main function.
fn main() {
    print_title();
    let (rule, mps, grid_size, square_size) = print_input_requests();
    println!("");
    println!("Rule = {}", rule);
    println!("Moves per second = {}", mps);
    println!("Grid size (number of squares) = {}", grid_size);
    println!("Square size (number of pixels) = {}", square_size);

    // Compute fps and moves_per_update control variables.
    let (fps, moves_per_tick) = match mps {
        1 => (1 as u64, 1 as i32),
        2 => (2 as u64, 1 as i32),
        5 => (5 as u64, 1 as i32),
        10 => (10 as u64, 1 as i32),
        20 => (20 as u64, 1 as i32),
        50 => (50 as u64, 1 as i32),
        100 => (10 as u64, 10 as i32),
        200 => (20 as u64, 10 as i32),
        500 => (50 as u64, 10 as i32),
        1000 => (50 as u64, 20 as i32),
        _ => (1 as u64, 1 as i32),
    };

    // Centre the starting point in the square grid.
    let start_point: usize = (grid_size as f64 / 2.0) as usize;

    // Initialise ant's position.
    let mut ant = Ant::new(start_point, start_point);
    ant.rule = Vec::with_capacity(rule.len());
    ant.colours = Vec::with_capacity(rule.len());

    // Build the route and colour vectors and store in Ant object
    const WHITE: Colour = Colour {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };

    for c in rule.chars() {
        if c == 'L' {
            ant.rule.push(Direction::L);
        } else if c == 'R' {
            ant.rule.push(Direction::R);
        }

        let mut col = create_random_colour();

        while WHITE.compare(&col) {
            col = create_random_colour();
        }

        ant.colours.push(col);
    }

    // Grid size in pixels will be multiplication of grid_size in squares
    // by square_size in pixels.
    let dim: u32 = grid_size * (square_size as u32);

    // Initialise Grid.
    let mut grid = Grid::new(grid_size as usize, grid_size as usize, usize::max_value());

    // Create our 2D render window.
    let mut window: PistonWindow = WindowSettings::new("Langton's Ant", [dim, dim])
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Tweak event loop timings.
    let mut evs = window.get_event_settings();
    evs.set_ups(fps);
    evs.set_max_fps(fps);
    window.set_event_settings(evs);

    // Process the events and start drawing.
    let ant_ref: &mut Ant = &mut ant;
    let grid_ref: &mut Grid = &mut grid;

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _device| {
            clear([1.0; 4], g);
            for _ in 0..moves_per_tick {
                compute_ant_position(ant_ref, grid_ref);
            }
            let mut x: u32 = 0;
            for row in &mut grid_ref.rows {
                let mut y: u32 = 0;
                for cell in &mut row.cells {
                    let xr = x as f64 * square_size;
                    let yr = y as f64 * square_size;
                    if *cell != usize::max_value() {
                        rectangle(
                            ant_ref.colours[*cell].to_rgba(),
                            [xr, yr, square_size, square_size],
                            c.transform,
                            g,
                        );
                    }
                    y += 1;
                }
                x += 1;
            }
        });

        let mut title = String::from("Langton's Ant - N = ");
        title.push_str(ant_ref.iterations.to_string().as_str());
        window.set_title(title);
    }
}
