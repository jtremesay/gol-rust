use gol::render::RenderType;
use gol::world::CellState;
use gol::world::World;

struct Settings {
    world_width: usize,
    world_height: usize,
    population_density: f32,
    run_steps_max: Option<usize>,
    render_type: RenderType,
    display_help: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            world_width: 320,
            world_height: 240,
            population_density: 0.5,
            run_steps_max: None,
            render_type: RenderType::Piston,
            display_help: false,
        }
    }
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
    println!("    --loop             Run the simulation forever (enabled by default)");
    println!("    --render type   The render to use (default piston) (available piston none");
}

enum ParseArgsError {
    MissingValue(String),
    InvalidValue(String, String),
    UnknowArg(String),
}

fn parse_args() -> Result<Settings, ParseArgsError> {
    let mut settings = Settings::default();

    let args: Vec<String> = std::env::args().collect();
    let mut arg_index = 1;

    while arg_index < args.len() {
        let current_arg = &args[arg_index];
        let next_arg = if arg_index + 1 == args.len() {
            None
        } else {
            Some(&args[arg_index + 1])
        };

        if current_arg == "--help" {
            settings.display_help = true;

            break;
        }

        if current_arg == "--width" {
            if let Some(width) = next_arg {
                settings.world_width = width.parse::<usize>().unwrap();

                // Consume the arg
                arg_index += 1;
            } else {
                return Err(ParseArgsError::MissingValue(current_arg.to_string()));
            }
        } else if current_arg == "--height" {
            if let Some(height) = next_arg {
                settings.world_height = height.parse::<usize>().unwrap();

                // Consume the arg
                arg_index += 1;
            } else {
                return Err(ParseArgsError::MissingValue(current_arg.to_string()));
            }
        } else if current_arg == "--density" {
            if let Some(density) = next_arg {
                settings.population_density = density.parse::<f32>().unwrap();

                // Consume the arg
                arg_index += 1;
            } else {
                return Err(ParseArgsError::MissingValue(current_arg.to_string()));
            }
        } else if current_arg == "--max-steps" {
            if let Some(max_steps) = next_arg {
                settings.run_steps_max = Some(max_steps.parse::<usize>().unwrap());

                // Consume the arg
                arg_index += 1;
            } else {
                return Err(ParseArgsError::MissingValue(current_arg.to_string()));
            }
        } else if current_arg == "--loop" {
            settings.run_steps_max = None;
        } else if current_arg == "--render" {
            if let Some(render) = next_arg {
                if render == "none" {
                    settings.render_type = RenderType::None;
                } else if render == "piston" {
                    settings.render_type = RenderType::Piston;
                } else {
                    return Err(ParseArgsError::InvalidValue(
                        current_arg.to_string(),
                        render.to_string(),
                    ));
                }

                // Consume the arg
                arg_index += 1;
            } else {
                return Err(ParseArgsError::MissingValue(current_arg.to_string()));
            }
        } else {
            return Err(ParseArgsError::UnknowArg(current_arg.to_string()));
        }

        arg_index += 1;
    }

    Ok(settings)
}

fn main() {
    // Parse the args
    let settings = parse_args().ok().unwrap();

    // Display the help if asked
    if settings.display_help {
        usage();

        return;
    }

    // Create the world
    let mut world = World::new(settings.world_width, settings.world_height);
    world.populate(settings.population_density);

    // Create the window if needed
    let mut window: Option<piston_window::PistonWindow> = match settings.render_type {
        RenderType::Piston => Some(
            piston_window::WindowSettings::new(
                "Game of Life",
                [settings.world_width as u32, settings.world_height as u32],
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
        println!("running step {}...", current_step);
        let step_start = std::time::SystemTime::now();

        if let Some(max_steps) = settings.run_steps_max {
            if current_step >= max_steps {
                break;
            }
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
            if let Some(window_) = window.as_mut() {
                if let Some(event) = window_.next() {
                    window_.draw_2d(&event, |context, graphics, _device| {
                        piston_window::clear([1.0; 4], graphics);

                        for y in 0..world.get_height() {
                            for x in 0..world.get_width() {
                                let cell_state = world.get_tile(x, y);
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
