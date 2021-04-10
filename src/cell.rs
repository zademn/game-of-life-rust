

// Structs and Implementations
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