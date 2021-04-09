# Game of life
This project is made to practice the rust language.


Below is a walkthrough through the project code.

# Imports
- We import `ggez` for our graphics and `rayon` to parallelize the update functionality
```rust
use ggez;
use ggez::event;
use ggez::event::EventHandler;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, ContextBuilder, GameResult};
use rayon::prelude::*;
```

# Settings
The settings used for our little game
```rust
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
```
# Structs and Logic
## The cell
> Keeps a `bool` as its state (if it's alive or not)
```rust
struct Cell {
    alive: bool,
}

impl Cell {
    pub fn new(alive: bool) -> Self {
        return Self { alive: alive };
    }
}
```

## Grid
> The grid has a width and height and we keep the number of cells in a `Vec<Cells>`

```rust
struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}
```

## State functions
- The `new` function creates a state based on a configuration given in settings


- The `set_state` function sets a given `Vec<Cells>` to alive and the rest to dead

```rust
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
```
### Update function
1. We get a `Vec<bool>` of `next_states` for each cell
2. We update the cells with the new cells

**Note**
- I used rayon for parallelisation, sequential code killed my fps when I tried bigger configurations
- Code can obviously be optimized
```rust
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
```

### Get next cell
Given a cell `idx` in the `Vec<Cells>` return a `bool` representing the next state
1. Get neighbours
2. Count alive neighbours
3. Get next state acording to the rules https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life


```rust
    fn cell_next_state(&self, cell_idx: usize) -> bool {
        let cell = self.cells[cell_idx].clone();
        let (cell_row, cell_col) = index_to_coords(cell_idx);
        // 1. Check boundaries and add neighgours
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

        // Count alive
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
```
# ggez
I used `ggez` for the graphics of this game since it's easy to use. We have 3 parts
## 1. main function setup
1. We build a new context `ctx` and we set the configuration (title, resolution etc)
2. We make a new `MainState` from our `ctx`
3. We run the event loop 
```rust
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
```

## 2. A `MainState`
> initializes, updates and draws

We initialize our game with a grid and a starting configuration with parameters given in the settings
```rust
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
```
## 3. `EventState` trait for the `MainState`
- We need to implement the `update` and `draw` functions now
```rust
impl EventHandler for MainState {
 //{...}
}
```
### `Update` function
- We set the fps using `ggez::timer::check_update_time(ctx, FPS)`
- We update the grid
```rust
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ggez::timer::check_update_time(ctx, FPS) {
            self.grid.update();
        }
        Ok(())
    }
```

### `Draw` function
1. Set the background color with `graphics::clear(ctx, graphics::BLACK);`
2. Make a mesh builder and add alive cells and a grid (if given) to it
3. Draw the mesh and present it to the screen
```rust
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
```
