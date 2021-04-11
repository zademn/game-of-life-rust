#[derive(Debug, Copy, Clone)]
pub struct Point{
    pub x: usize,
    pub y: usize,
}

impl From<(usize, usize)> for Point{
    fn from(item: (usize, usize)) -> Self{
        Self {x: item.0, y: item.1}
    }
}
