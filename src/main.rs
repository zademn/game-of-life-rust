mod cell;
mod grid;
mod types;

use crate::grid::Grid;
use crate::types::Point;
use clap::{App, Arg};

use ggez::event;
use ggez::event::EventHandler;
use ggez::graphics;
use ggez::{Context, ContextBuilder, GameResult};
use rand::Rng;

const GRID: bool = false;
//const CELL_SIZE: f32 = SCREEN_SIZE.0 / GRID_WIDTH as f32;

#[allow(dead_code)]
const BLINKER: [(usize, usize); 3] = [(4, 4), (4, 5), (4, 6)];
#[allow(dead_code)]
const TOAD: [(usize, usize); 6] = [(4, 4), (4, 5), (4, 6), (5, 3), (5, 4), (5, 5)];
#[allow(dead_code)]
const GLIDER: [(usize, usize); 5] = [(1, 2), (3, 2), (2, 3), (3, 3), (2, 4)];
#[allow(dead_code)]
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

/// Config for the start of the game
#[derive(Debug, Clone)]
pub struct Config {
    pub grid_width: usize,
    pub grid_height: usize,
    pub cell_size: f32,
    pub screen_size: (f32, f32),
    pub fps: u32,
    pub initial_state: String,
}

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
        MainState {
            grid,
            config,
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ggez::timer::check_update_time(ctx, self.config.fps) {
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

fn main() -> GameResult {
    // CLI
    let matches = App::new("Game of Life")
        .version("0.1")
        .author("Zademn")
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .help("Grid width")
                .value_name("width")
                .takes_value(true)
                .required(false)
                .default_value("64"),
        )
        .arg(
            Arg::with_name("height")
                .short("h")
                .long("height")
                .help("Grid height")
                .value_name("height")
                .takes_value(true)
                .required(false)
                .default_value("64"),
        )
        .arg(
            Arg::with_name("initial_state")
                .short("s")
                .long("initial-state")
                .help("Initial state options: blinker, toad, glider, glider-gun, random")
                .value_name("initial_state")
                .takes_value(true)
                .required(false)
                .default_value("random"),
        )
        .get_matches();

    // Get Configurations
    let grid_width = matches.value_of("width").unwrap().parse::<usize>().unwrap();
    let grid_height = matches
        .value_of("height")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let initial_state = matches.value_of("initial_state").unwrap();
    let screen_size = (720., 720.);
    let fps = 20;
    // Set configuration
    let config: Config = Config {
        grid_width,
        grid_height,
        cell_size: screen_size.0 / grid_width as f32,
        screen_size,
        fps,
        initial_state: initial_state.to_string(),
    };

    // Setup ggez stuff
    let cb = ContextBuilder::new("Game of life", "Zademn")
        .window_mode(ggez::conf::WindowMode::default().dimensions(screen_size.0, screen_size.1));
    let (ctx, event_loop) = &mut cb.build()?; // `?` because the build function may fail
    graphics::set_window_title(ctx, "Game of life");
    // Setup game state -> game loop
    let mut state = MainState::new(ctx, config);
    event::run(ctx, event_loop, &mut state)?;
    Ok(())
}
