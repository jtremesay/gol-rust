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

enum RenderBackend {
    None,
    Piston,
}

trait Render {
    fn render(&mut self, world: &World);
}

struct NoneRenderBackend {}

impl NoneRenderBackend {
    fn new() -> Self {
        Self {}
    }
}

impl Render for NoneRenderBackend {
    fn render(&mut self, _: &World) {}
}

struct PistonRenderBackend {}

impl PistonRenderBackend {
    fn new() -> Self {
        Self {}
    }
}

impl Render for PistonRenderBackend {
    fn render(&mut self, _: &World) {}
}

fn usage() {
    println!("Usage: gol [--help] [--width width] [--height height] [--max-steps steps]");
    println!("");
    println!("Options");
    println!("    --help             Display this message");
    println!("    --width width      Define the size of the world (default 320)");
    println!("    --height height    Define the height of the world (default 240)");
    println!("    --density density  Define the initial density of population of the world (default 0.5)");
    println!("    --max-steps steps  The number of steps to run of the simulation (default 0)");
    println!("    --loop steps       Run the simulation for ever (enabled by default)");
    println!(
        "    --render backend   The render backend to use (default piston) (available piston none"
    );
}

fn main() {
    // Parse args
    let args: Vec<String> = std::env::args().collect();
    let mut world_width = 320;
    let mut world_height = 240;
    let mut world_density = 0.5;
    let mut max_steps = 0;
    let mut run_forever = true;
    let mut display_help = false;
    let mut arg_index = 1;
    let mut render_backend_type = RenderBackend::Piston;
    while arg_index < args.len() {
        let current_arg = &args[arg_index];
        let next_arg = if arg_index + 1 == args.len() {
            None
        } else {
            Some(&args[arg_index + 1])
        };

        if current_arg == "--help" {
            display_help = true;

            break;
        }

        if current_arg == "--width" {
            if let Some(width) = next_arg {
                world_width = width.parse::<usize>().unwrap();

                // Consume the arg
                arg_index += 1;
            } else {
                panic!("Missing value for parameter --width")
            }
        } else if current_arg == "--height" {
            if let Some(height) = next_arg {
                world_height = height.parse::<usize>().unwrap();

                // Consume the arg
                arg_index += 1;
            } else {
                panic!("Missing value for parameter --height")
            }
        } else if current_arg == "--density" {
            if let Some(density) = next_arg {
                world_density = density.parse::<f32>().unwrap();

                // Consume the arg
                arg_index += 1;
            } else {
                panic!("Missing value for parameter --density")
            }
        } else if current_arg == "--max-steps" {
            if let Some(max_steps_) = next_arg {
                max_steps = max_steps_.parse::<usize>().unwrap();
                run_forever = false;

                // Consume the arg
                arg_index += 1;
            } else {
                panic!("Missing value for parameter --density")
            }
        } else if current_arg == "--loop" {
            max_steps = 0;
            run_forever = true;
        } else if current_arg == "--render" {
            if let Some(render) = next_arg {
                if render == "none" {
                    render_backend_type = RenderBackend::None;
                } else if render == "piston" {
                    render_backend_type = RenderBackend::Piston;
                } else {
                    panic!("Unknow value {} for parameter --render", render);
                }

                // Consume the arg
                arg_index += 1;
            } else {
                panic!("Missing value for parameter --render")
            }
        } else {
            panic!("Unexpected remaining argument {}", current_arg)
        }

        arg_index += 1;
    }

    // Display the help if asked
    if display_help {
        usage();

        return;
    }

    // Create the world
    let mut world = World::new(world_width, world_height);
    world.populate(world_density);

    // Create the window if needed
    let mut window: Option<piston_window::PistonWindow> = match render_backend_type {
        RenderBackend::Piston => Some(
            piston_window::WindowSettings::new(
                "Hello Piston!",
                [world_width as u32, world_height as u32],
            )
            .exit_on_esc(true)
            .build()
            .unwrap(),
        ),
        _ => None,
    };

    // Main loop
    let mut current_step = 0;
    //while let Some(event) = window.next() {
    loop {
        println!("running step {}_", current_step);
        let step_start = std::time::SystemTime::now();

        if !run_forever && current_step == max_steps {
            break;
        }

        // Update the world
        {
            println!("update world...");
            let update_start = std::time::SystemTime::now();
            world.update();
            let update_end = std::time::SystemTime::now();
            let update_duration = update_end.duration_since(update_start).unwrap();
            println!("update done, took {:?}", update_duration);
        }

        // Render the world
        {
            println!("render world...");
            let render_start = std::time::SystemTime::now();
            if let Some(mut window_) = window.as_mut() {
                if let Some(event) = window_.next() {
                    window_.draw_2d(&event, |context, graphics, _device| {
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
            let render_end = std::time::SystemTime::now();
            let render_duration = render_end.duration_since(render_start).unwrap();
            println!("render done, took {:?}", render_duration);
        }

        let step_end = std::time::SystemTime::now();
        let step_duration = step_end.duration_since(step_start).unwrap();
        println!(
            "step done, took {:?} ({:.0} FPS)",
            step_duration,
            1.0 / step_duration.as_secs_f64()
        );

        current_step += 1;
    }
}
