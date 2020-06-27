use gol::render::RenderBackend;
use gol::world::CellState;
use gol::world::World;

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
                "Game of Life",
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
        println!("running step {}...", current_step);
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
