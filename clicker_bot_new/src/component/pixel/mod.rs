use color_eyre::eyre::Report;
use device_query::{DeviceQuery, DeviceState};
use image::io::Reader as ImageReader;
use image::GenericImageView;
use log::debug;
use screenshots::Screen;
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::fs::write;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use tempfile::NamedTempFile;

#[allow(dead_code)]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Pixel {
    x: usize,
    y: usize,
    rgba: [u8; 4],
}

pub fn select(screen: Screen) -> Result<Pixel, Report> {
    let device_state = DeviceState::new();
    let mut pixel_selected = false;
    let mut x = 0;
    let mut y = 0;
    let wait = Duration::from_millis(10);

    let mut mouse = device_state.get_mouse();
    let mut left_click = mouse.button_pressed[1];

    // If the left button is already pressed, wait until it's released
    while left_click {
        mouse = device_state.get_mouse();
        left_click = mouse.button_pressed[1];
    }

    // Now that the left button is released, wait for it to be clicked again
    while !pixel_selected {
        mouse = device_state.get_mouse();
        left_click = mouse.button_pressed[1];

        x = mouse.coords.0;
        y = mouse.coords.1;

        if left_click {
            pixel_selected = true;
        }
        thread::sleep(wait);
    }

    // Get pixel color at coord
    let screen_capture = screen.capture_area(x, y, 1, 1).unwrap();
    let buffer = screen_capture.buffer();

    // Write capture to file
    let tmp_file = NamedTempFile::new()?;
    //let tmp_file = tmp_file.path().to_str().unwrap();
    let mut tmp_file = PathBuf::from(tmp_file.path());
    tmp_file.set_extension("png");
    write(tmp_file.clone(), buffer)?;

    // Parse img to pixel
    let img = ImageReader::open(&tmp_file)?.decode()?;
    let rgba = img.get_pixel(0, 0);
    // Convert to array, for deserialize
    let rgba = [rgba[0], rgba[1], rgba[2], rgba[3]];
    let pixel = Pixel {
        x: x as usize,
        y: y as usize,
        rgba,
    };
    debug!("{pixel:?}");

    Ok(pixel)
}
