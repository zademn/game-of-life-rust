#![allow(dead_code)]
use ggez;
use ggez::event;
use ggez::event::EventHandler;
use ggez::graphics;
use ggez::{Context, ContextBuilder, GameResult};
use rayon::prelude::*;

const SCREEN_SIZE: (f32, f32) = (640., 640.);
const GRID_HEIGHT: usize = 64;
const GRID_WIDTH: usize = 64;
const GRID: bool = false;
const FPS: u32 = 20;
const CELL_SIZE: f32 = SCREEN_SIZE.0 / GRID_WIDTH as f32;

const BLINKER: [(usize, usize); 3] = [(4, 4), (4, 5), (4, 6)];
const TOAD: [(usize, usize); 6] = [(4, 4), (4, 5), (4, 6), (5, 3), (5, 4), (5, 5)];
const GLIDER: [(usize, usize); 5] = [(2, 1), (2, 3), (3, 2), (3, 3), (4, 2)];
const GLIDER_GUN: [(usize, usize); 36] = [
    (5, 1),
    (5, 2),
    (6, 1),
    (6, 2),
    (5, 11),
    (6, 11),
    (7, 11),
    (4, 12),
    (3, 13),
    (3, 14),
    (8, 12),
    (9, 13),
    (9, 14),
    (6, 15),
    (4, 16),
    (5, 17),
    (6, 17),
    (7, 17),
    (6, 18),
    (8, 16),
    (3, 21),
    (4, 21),
    (5, 21),
    (3, 22),
    (4, 22),
    (5, 22),
    (2, 23),
    (6, 23),
    (1, 25),
    (2, 25),
    (6, 25),
    (7, 25),
    (3, 35),
    (4, 35),
    (3, 36),
    (4, 36),
];

const CONFIG_0: [(usize, usize); 36] = [
    (50, 180),
    (51, 180),
    (50, 181),
    (51, 181),
    (60, 180),
    (60, 179),
    (60, 181),
    (61, 178),
    (62, 177),
    (63, 177),
    (61, 182),
    (62, 183),
    (63, 183),
    (65, 182),
    (66, 181),
    (66, 180),
    (66, 179),
    (65, 178),
    (64, 180),
    (67, 180),
    (70, 181),
    (70, 182),
    (70, 183),
    (71, 181),
    (71, 182),
    (71, 183),
    (72, 180),
    (72, 184),
    (74, 180),
    (74, 179),
    (74, 184),
    (74, 185),
    (84, 182),
    (84, 183),
    (85, 182),
    (85, 183),
];
// Utils

/// Converts a pair of cell coords to index in the cells vector
pub fn coords_to_index(row: usize, column: usize) -> usize {
    return row * GRID_WIDTH + column;
}

/// Converts a index in the cells vecotr into pair of cell coords
pub fn index_to_coords(index: usize) -> (usize, usize) {
    return (index / GRID_WIDTH, index % GRID_HEIGHT);
}

// Structs and Implementations
#[derive(Clone, Debug)]
struct Cell {
    alive: bool,
}

impl Cell {
    pub fn new(alive: bool) -> Self {
        return Self { alive: alive };
    }
}

struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Grid {
    // Width and height of the Grid
    pub fn new(width: usize, height: usize) -> Self {
        return Self {
            width: width,
            height: height,
            cells: vec![Cell::new(false); width * height],
        };
    }
    pub fn set_state(&mut self, cells_coords: &[(usize, usize)]) {
        self.cells = vec![Cell::new(false); self.width * self.height];
        for (row, col) in cells_coords.iter() {
            let idx = coords_to_index(*row, *col);
            self.cells[idx].alive = true;
        }
    }
    fn cell_next_state(&self, cell_idx: usize) -> bool {
        let cell = self.cells[cell_idx].clone();
        let (cell_row, cell_col) = index_to_coords(cell_idx);
        // Check boundaries and add neighgours
        let mut neighbours_vec = vec![];
        for &x_off in [-1, 0, 1].iter() {
            for &y_off in [-1, 0, 1].iter() {
                if x_off == 0 && y_off == 0 {
                    continue;
                }
                let neighbour_coords = (cell_row as isize + x_off, cell_col as isize + y_off);
                if neighbour_coords.0 < 0
                    || neighbour_coords.0 > GRID_WIDTH as isize - 1
                    || neighbour_coords.1 < 0
                    || neighbour_coords.1 > GRID_HEIGHT as isize - 1
                {
                    continue;
                }
                neighbours_vec.push(neighbour_coords);
            }
        }
        let mut num_neighbour_alive = 0;

        for (row, col) in neighbours_vec.iter() {
            let idx = coords_to_index(*row as usize, *col as usize);

            if self.cells[idx].alive {
                num_neighbour_alive += 1;
            }
        }

        // Rules (from wikipedia)
        if cell.alive && (num_neighbour_alive == 2 || num_neighbour_alive == 3) {
            return true; // alive
        }
        if cell.alive == false && num_neighbour_alive == 3 {
            return true;
        }

        return false;
    }
    pub fn update(&mut self) {
        // Vector of next states. It will match by index
        // Get next states
        // Iterative lags, parallel stronk
        // let mut next_states = vec![false; self.cells.len()];
        // for idx in (0..self.cells.len()) {
        //     let next_state = self.cell_next_state(idx);
        //     next_states[idx] = next_state;
        // }
        let next_states = (0..self.cells.len())
            .into_par_iter()
            .map(|idx| {
                let next_state = self.cell_next_state(idx);
                //next_states[idx] = next_state;
                next_state
            })
            .collect::<Vec<bool>>();

        // Update states
        // for idx in 0..self.cells.len() {
        //     self.cells[idx].alive = next_states[idx];
        // }
        self.cells = (0..self.cells.len())
            .into_par_iter()
            .map(|idx| {
                //self.cells[idx].alive = next_states[idx];
                Cell {
                    alive: next_states[idx],
                }
            })
            .collect::<Vec<Cell>>();
    }
}

struct MainState {
    grid: Grid,
}
impl MainState {
    pub fn new(ctx: &mut Context) -> Self {
        let mut grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);
        let start_cells_coords = GLIDER_GUN;
        grid.set_state(&start_cells_coords);
        return MainState { grid: grid };
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ggez::timer::check_update_time(ctx, FPS) {
            self.grid.update();
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);
        // Mesh builder
        let mut builder = graphics::MeshBuilder::new();
        // Init, otherwise doesn't work for some reason
        builder.rectangle(
            graphics::DrawMode::fill(),
            graphics::Rect::new(0., 0., 0., 0.),
            graphics::BLACK,
        );
        // Draw cells
        for (idx, cell) in self.grid.cells.iter().enumerate() {
            if cell.alive {
                let (row, col) = index_to_coords(idx);
                let color = graphics::Color::new(0., 200., 0., 1.); // Green
                builder.rectangle(
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(
                        col as f32 * CELL_SIZE,
                        row as f32 * CELL_SIZE,
                        CELL_SIZE,
                        CELL_SIZE,
                    ),
                    color,
                );
            }
        }
        // Draw grid
        if GRID {
            for idx in 0..self.grid.cells.len() {
                let color = graphics::Color::new(10., 10., 10., 1.); // ?
                let (row, col) = index_to_coords(idx);
                builder.rectangle(
                    graphics::DrawMode::stroke(1.),
                    graphics::Rect::new(
                        col as f32 * CELL_SIZE,
                        row as f32 * CELL_SIZE,
                        CELL_SIZE,
                        CELL_SIZE,
                    ),
                    color,
                );
            }
        }
        let mesh = builder.build(ctx)?;
        // Draw
        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        // Present on screen
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    // Setup stuff
    let cb = ContextBuilder::new("Game of life", "Zademn")
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1));
    let (ctx, event_loop) = &mut cb.build()?; // `?` because the build function may fail
    graphics::set_window_title(ctx, "Game of life");
    // Setup game state -> game loop
    let mut state = MainState::new(ctx);
    event::run(ctx, event_loop, &mut state)?;
    Ok(())
}
