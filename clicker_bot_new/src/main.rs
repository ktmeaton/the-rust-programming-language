use color_eyre::eyre::{Result, Report};
use device_query::{DeviceQuery, DeviceState};
use image::io::Reader as ImageReader;
use screenshots::Screen;
use std::default::Default;
use std::time::Duration;
use std::thread;

#[derive(Debug)]
struct Bot {
    wait: Duration,
}

impl Default for Bot {
    fn default() -> Self {
        Bot {
            wait: Duration::from_millis(10)
        }
    }
}

#[derive(Debug)]
struct Human {
    mouse: Mouse,
}

impl Default for Human {
    fn default() -> Self {
        Human {
            mouse: Mouse::default(),
        }
    }    
}

#[derive(Debug)]
struct Mouse {
    left_click: bool,
    x: i32,
    y: i32,
}

impl Default for Mouse {
    fn default() -> Self {
        Mouse {
            left_click: false,
            x : 0,
            y : 0,
        }
    }    
}

fn main () -> Result<(), Report> {

    color_eyre::install()?;
    // Initialize devices
    // Screen, keys, mouse

    // Screen
    let screens = Screen::all().unwrap();
    let screen = screens[0];

    let device_state = DeviceState::new();

    let mut bot = Bot::default();
    let mut human = Human::default();
    let mut mouse = device_state.get_mouse();

    println!("Click on the target monster pixel.");

    let mut pixel_selected = false;
    while !pixel_selected {

        mouse = device_state.get_mouse();
        human.mouse.left_click = mouse.button_pressed[1];
        human.mouse.x = mouse.coords.0;
        human.mouse.y = mouse.coords.1;

        if human.mouse.left_click {
            println!("\tCoordinates: {} {}", human.mouse.x, human.mouse.y);
            pixel_selected = true;
        }

        thread::sleep(bot.wait);
    }

    // Get pixel color at coord
    println!("{:?}", human);

    let screens = Screen::all().unwrap();
    let screen = screens[0];

    let pixel = screen.capture_area(human.mouse.x, human.mouse.y, 1, 1).unwrap();
    let buffer = pixel.buffer();
    std::fs::write("pixel.png", buffer).unwrap();
    let img = ImageReader::open("pixel.png")?.decode()?;
    println!("{:?}", img);

    Ok(())

}