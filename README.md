# Game of life

I made this project to practice the rust language.

Below is a walkthrough for the project's code.

- cell.rs
- grid.rs
- types.rs
- main.rs

# Imports

- We import `ggez` for our graphics and `rayon` to parallelize the update functionality. We use `clap` for command line arguments
- https://github.com/ggez/ggez
- https://github.com/rayon-rs/rayon
- https://github.com/clap-rs/clap

```rust
use crate::grid::Grid;
use crate::types::Point;
use clap::{App, Arg};
use ggez;
use ggez::event;
use ggez::event::EventHandler;
use ggez::graphics;
use ggez::{Context, ContextBuilder, GameResult};
use rand::Rng;
```

# Structs and Logic

## The point

> A point structure that stores the `x` and `y` coordinate. We will use `(usize, usize).into()` to convert it fast

- Defined in types.rs

```rust
#[derive(Debug, Copy, Clone)]
pub struct Point{
    pub x: usize,
    pub y: usize,
}

impl From<(usize, usize)> for Point{
    fn from(item: (usize, usize)) -> Self{
        return Self {x: item.0, y: item.1};
    }
}
```

## The cell

> Keeps a `bool` as its state (if it's alive or not)

- Defined in cell.rs

```rust
#[derive(Clone, Debug)]
pub struct Cell {
    alive: bool,
}

impl Cell {
    pub fn new(alive: bool) -> Self {
        return Self { alive: alive };
    }
    pub fn is_alive(&self) -> bool {
        return self.alive;
    }
    pub fn set_state(&mut self, state: bool){
        self.alive = state;
    }
}
```

## The Grid

> The grid has a width and height and we keep the cells in a `Vec<Cells>`

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
impl Grid {
    // Width and height of the Grid
    pub fn new(width: usize, height: usize) -> Self {
        return Self {
            width: width,
            height: height,
            cells: vec![Cell::new(false); width * height],
        };
    }
    pub fn set_state(&mut self, cells_coords: &[Point]) {
        self.cells = vec![Cell::new(false); self.width * self.height];
        for &pos in cells_coords.iter() {
            let idx = self.coords_to_index(pos);
            self.cells[idx].set_state(true);
        }
    }
}
```

### Update function

1. We get a `Vec<bool>` of `next_states` for each cell
2. We update the cells with the new cells

**Note**

- I used `rayon` for parallelization since sequential code killed my fps when I tried bigger configurations
- Obviously, code can be optimized

```rust
impl Grid{
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
            .map(|idx| Cell::new(next_states[idx]))
            .collect::<Vec<Cell>>();
    }
}
```

### Get next cell

Given a cell `idx` in the `Vec<Cells>` return a `bool` representing the next state

1. Count alive neighbours
2. Get next state acording to the rules https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life

```rust
impl Grid{
    fn cell_next_state(&self, cell_idx: usize) -> bool {
        let cell = self.cells[cell_idx].clone();
        let cell_pos = self.index_to_coords(cell_idx);
        // Check boundaries and add neighgours
        let mut num_neighbour_alive = 0;
        for &x_off in [-1, 0, 1].iter() {
            for &y_off in [-1, 0, 1].iter() {
                if x_off == 0 && y_off == 0 {
                    continue;
                }
                let neighbour_pos;
                let neighbour_coords = (cell_pos.x as isize + x_off, cell_pos.y as isize + y_off);

                // Make torus
                if neighbour_coords.0 < 0 {
                    // top-left cell
                    if neighbour_coords.1 < 0 {
                        neighbour_pos = Point {
                            x: self.width - 1,
                            y: self.height - 1,
                        }
                    } else if neighbour_coords.1 > self.height as isize - 1 {
                        // bottom-left cell
                        neighbour_pos = Point {
                            x: self.width - 1,
                            y: 0,
                        }
                    } else {
                        // left cell
                        neighbour_pos = Point {
                            x: self.width - 1,
                            y: neighbour_coords.1 as usize,
                        }
                    }
                } else if neighbour_coords.0 > self.width as isize - 1 {
                    if neighbour_coords.1 < 0 {
                        // top-right cell
                        neighbour_pos = Point {
                            x: 0,
                            y: self.height - 1,
                        }
                    } else if neighbour_coords.1 > self.height as isize - 1 {
                        // bottom-right cell
                        neighbour_pos = Point { x: 0, y: 0 }
                    } else {
                        // right cell
                        neighbour_pos = Point {
                            x: 0,
                            y: neighbour_coords.1 as usize,
                        }
                    }
                } else if neighbour_coords.1 < 0 {
                    // top cell
                    neighbour_pos = Point {
                        x: neighbour_coords.0 as usize,
                        y: self.height - 1,
                    }
                } else if neighbour_coords.1 > self.height as isize - 1 {
                    // bottom cell
                    neighbour_pos = Point {
                        x: neighbour_coords.0 as usize,
                        y: 0,
                    }
                } else {
                    // Others cells
                    neighbour_pos = Point {
                        x: neighbour_coords.0 as usize,
                        y: neighbour_coords.1 as usize,
                    };
                }

                let idx = self.coords_to_index(neighbour_pos);
                if self.cells[idx].is_alive() {
                    num_neighbour_alive += 1;
                }
            }
        }

        // Rules https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life
        if cell.is_alive() && (num_neighbour_alive == 2 || num_neighbour_alive == 3) {
            return true; // alive
        }
        if !cell.is_alive() && num_neighbour_alive == 3 {
            return true;
        }

        false
    }
```

# Clap and CLI

- We keep the configurations in a `struct Config`
  From the help:

```
USAGE:
    game_of_life.exe [OPTIONS]

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -h, --height <height>                  Grid height [default: 64]
    -s, --initial-state <initial_state>    Initial state options: blinker, toad, glider, glider-gun, random [default:
                                           random]
    -w, --width <width>                    Grid width [default: 64]
```

```rust
#[derive(Debug, Clone)]
pub struct Config {
    pub grid_width: usize,
    pub grid_height: usize,
    pub cell_size: f32,
    pub screen_size: (f32, f32),
    pub fps: u32,
    pub initial_state: String,
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
    config: Config,
}

impl MainState {
    pub fn new(_ctx: &mut Context, config: Config) -> Self {
        // Initialize the grid based on configuration
        let mut grid = Grid::new(config.grid_width, config.grid_height);
        // Initialize starting configuration
        let mut start_cells_coords: Vec<Point> = vec![];
        match &config.initial_state[..] {
            "glider-gun" => {
                start_cells_coords = GLIDER_GUN.iter().map(|&p| p.into()).collect::<Vec<Point>>();
            }
            "toad" => {
                start_cells_coords = TOAD.iter().map(|&p| p.into()).collect::<Vec<Point>>();
            }
            "glider" => {
                start_cells_coords = GLIDER.iter().map(|&p| p.into()).collect::<Vec<Point>>();
            }
            "blinker" => {
                start_cells_coords = BLINKER.iter().map(|&p| p.into()).collect::<Vec<Point>>();
            }
            _ => {
                let mut rng = rand::thread_rng();
                for i in 0..config.grid_width{
                    for j in 0..config.grid_height{
                        if rng.gen::<bool>(){
                            start_cells_coords.push((i, j).into());
                        }
                    }
                }
            }
        }
        // Convert the starting states into a vector of points
        grid.set_state(&start_cells_coords);
        return MainState {
            grid: grid,
            config: config,
        };
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
impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ggez::timer::check_update_time(ctx, FPS) {
            self.grid.update();
        }
        Ok(())
    }
}
```

### `Draw` function

1. Set the background color with `graphics::clear(ctx, graphics::BLACK);`
2. Make a mesh builder and add alive cells and a grid (if given) to it
3. Draw the mesh and present it to the screen

```rust
impl EventHandler for MainState {
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
            if cell.is_alive() {
                let pos = self.grid.index_to_coords(idx);
                let color = graphics::Color::new(0., 200., 0., 1.); // Green
                builder.rectangle(
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(
                        pos.x as f32 * self.config.cell_size,
                        pos.y as f32 * self.config.cell_size,
                        self.config.cell_size,
                        self.config.cell_size,
                    ),
                    color,
                );
            }
        }
        // Draw grid
        if GRID {
            for idx in 0..self.grid.cells.len() {
                let color = graphics::Color::new(10., 10., 10., 1.); // ?
                let pos = self.grid.index_to_coords(idx);
                builder.rectangle(
                    graphics::DrawMode::stroke(1.),
                    graphics::Rect::new(
                        pos.x as f32 * self.config.cell_size,
                        pos.y as f32 * self.config.cell_size,
                        self.config.cell_size,
                        self.config.cell_size,
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
```
