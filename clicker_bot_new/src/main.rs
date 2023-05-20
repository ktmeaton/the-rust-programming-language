use clicker_bot::component::pixel;
use color_eyre::eyre::{Result, Report};
use log::info;
use screenshots::Screen;
use std::env;
use std::time::Duration;

// #[derive(Debug)]
// struct Bot {
//     wait: Duration,
// }

// impl Default for Bot {
//     fn default() -> Self {
//         Bot {
//             wait: Duration::from_millis(10)
//         }
//     }
// }

// #[derive(Debug)]
// struct Human {
//     mouse: Mouse,
// }

// impl Default for Human {
//     fn default() -> Self {
//         Human {
//             mouse: Mouse::default(),
//         }
//     }
// }

// #[derive(Debug)]
// struct Mouse {
//     left_click: bool,
//     x: i32,
//     y: i32,
// }

// impl Default for Mouse {
//     fn default() -> Self {
//         Mouse {
//             left_click: false,
//             x : 0,
//             y : 0,
//         }
//     }
// }

fn main () -> Result<(), Report> {

    let verbosity = "debug";

    // Set default logging level if RUST_LOG is not set.
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", verbosity)
    }

    env_logger::init();
    color_eyre::install()?;

    // Initialize devices
    // Screen, keys, mouse

    // Screen
    let screens = Screen::all().unwrap();
    let screen = screens[0];

    // let device_state = DeviceState::new();

    // let mut bot = Bot::default();
    // let mut human = Human::default();
    // let mut mouse = device_state.get_mouse();

    // Monster
    info!("Click on the target monster pixel.");
    let wait = Duration::from_millis(10);
    let _pixel = pixel::select(screen, wait)?;

    Ok(())

}
