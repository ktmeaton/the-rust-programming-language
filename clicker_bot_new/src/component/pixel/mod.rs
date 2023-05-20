use color_eyre::eyre::Report;
use device_query::{DeviceQuery, DeviceState};
use image::{GenericImageView, Rgba};
use image::io::Reader as ImageReader;
use log::debug;
use std::fs::write;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use screenshots::Screen;
use tempfile::NamedTempFile;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Pixel {
    x : usize,
    y : usize,
    rgba : Rgba<u8>,
}

pub fn select(screen: Screen, wait: Duration) -> Result<Pixel, Report> {

    let device_state = DeviceState::new();
    let mut pixel_selected = false;
    let mut x = 0;
    let mut y = 0;

    while !pixel_selected {

        let mouse = device_state.get_mouse();
        let left_click = mouse.button_pressed[1];

        x = mouse.coords.0;
        y = mouse.coords.1;

        if left_click { pixel_selected = true; }
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
    let pixel = Pixel {
        x: x as usize,
        y: y as usize,
        rgba
    };
    debug!("{pixel:?}");

    Ok(pixel)

}
