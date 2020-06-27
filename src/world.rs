/// The state of cell
#[derive(Clone, Copy, PartialEq)]
pub enum CellState {
    /// A dead cell
    Dead,
    /// An alive cell
    Alive,
}

/// A world
pub struct World {
    /// Width of the world
    width: usize,
    /// Height of the world
    height: usize,
    /// Tiles of the world
    tiles: Vec<Vec<CellState>>,
}

impl World {
    /// Create a new world
    ///
    /// @param width Width of the world
    /// @param height Height of the world
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            tiles: vec![vec![CellState::Dead; width]; height],
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_tile(&self, x: usize, y: usize) -> CellState {
        self.tiles[y][x]
    }

    pub fn set_tile(&mut self, x: usize, y: usize, cell_state: CellState) {
        self.tiles[y][x] = cell_state;
    }

    /// Populate the world randomly
    ///
    /// @param density The population density
    pub fn populate(&mut self, density: f32) {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell_state = if rand::random::<f32>() < density {
                    CellState::Alive
                } else {
                    CellState::Dead
                };
                self.tiles[y][x] = cell_state;
            }
        }
    }

    /// Update the world
    pub fn update(&mut self) {
        let mut new_tiles = vec![vec![CellState::Dead; self.width]; self.height];

        for y in 0..self.height {
            for x in 0..self.width {
                let cell_state = self.tiles[y][x];

                let left_x = if x == 0 { self.width - 1 } else { x - 1 };
                let right_x = if x == self.width - 1 { 0 } else { x + 1 };
                let top_y = if y == self.height - 1 { 0 } else { y + 1 };
                let bottom_y = if y == 0 { self.height - 1 } else { y - 1 };

                let neighbors_count = [
                    // Top left
                    (left_x, top_y),
                    // Top
                    (x, top_y),
                    // Top right
                    (right_x, top_y),
                    // Left
                    (left_x, y),
                    // Right
                    (right_x, y),
                    // Bottom left
                    (left_x, bottom_y),
                    // Bottom
                    (x, bottom_y),
                    // Bottom right
                    (right_x, bottom_y),
                ]
                .iter()
                .map(|(x, y)| self.tiles[*y][*x])
                .filter(|cell_state| match cell_state {
                    CellState::Alive => true,
                    _ => false,
                })
                .count();

                let new_state = if neighbors_count == 3
                    || (neighbors_count == 2 && cell_state == CellState::Alive)
                {
                    CellState::Alive
                } else {
                    CellState::Dead
                };

                new_tiles[y][x] = new_state;
            }
        }

        self.tiles = new_tiles;
    }
}
