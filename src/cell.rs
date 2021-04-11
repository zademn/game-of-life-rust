

// Structs and Implementations
#[derive(Clone, Debug)]
pub struct Cell {
    alive: bool,
}

impl Cell {
    pub fn new(alive: bool) -> Self {
        Self { alive }
    }
    pub fn is_alive(&self) -> bool {
        self.alive
    }
    pub fn set_state(&mut self, state: bool){
        self.alive = state;
    }
}