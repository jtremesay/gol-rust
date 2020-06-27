/// The state of cell
#[derive(Clone, Copy, PartialEq)]
enum CellState {
    /// A dead cell
    Dead,
    /// An alive cell
    Alive,
}

/// A world
struct World {
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
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            tiles: vec![vec![CellState::Dead; width]; height],
        }
    }

    /// Populate the world randomly
    ///
    /// @param density The population density
    fn populate(&mut self, density: f32) {
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
    fn update(&mut self) {
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

fn main() {
    // Configuration
    let world_width = 320;
    let world_height = 240;
    let world_density = 0.5;
    let max_steps = 100;

    // Create the world
    let mut world = World::new(world_width, world_height);
    world.populate(world_density);

    // Create the main window
    let mut window: piston_window::PistonWindow = piston_window::WindowSettings::new(
        "Hello Piston!",
        [world_width as u32, world_height as u32],
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    // Main loop
    //while let Some(event) = window.next() {
    for _ in 0..max_steps {
        if let Some(event) = window.next() {
            // Update the world
            world.update();

            // Do the render
            window.draw_2d(&event, |context, graphics, _device| {
                piston_window::clear([1.0; 4], graphics);

                for y in 0..world.height {
                    for x in 0..world.width {
                        let cell_state = world.tiles[y][x];
                        if cell_state == CellState::Alive {
                            piston_window::rectangle(
                                [0.0, 0.0, 0.0, 1.0],
                                [x as f64, y as f64, 1.0, 1.0],
                                context.transform,
                                graphics,
                            );
                        }
                    }
                }
            });
        }
    }
}
