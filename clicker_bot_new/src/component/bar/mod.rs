use crate::component::{pixel, pixel::Pixel};
use crate::traits::ToYaml;
use color_eyre::eyre::Report;
use log::info;
use screenshots::Screen;
use serde::{Deserialize, Serialize};
use std::default::Default;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Bar {
    pub name: String,
    pub empty: Pixel,
    pub full: Pixel,
}

impl Default for Bar {
    fn default() -> Self {
        Bar {
            name: "default".to_string(),
            empty: Pixel::default(),
            full: Pixel::default(),
        }
    }
}

impl ToYaml for Bar {}

pub fn select(name: &str) -> Result<Bar, Report> {
    let screens = Screen::all().unwrap();
    let screen = screens[0];

    info!("Click on the {name} bar at full.");
    let full = pixel::select(screen)?;
    info!("Click on the {name} bar at empty.");
    let empty = pixel::select(screen)?;
    let bar = Bar {
        name: name.to_string(),
        empty,
        full,
    };

    Ok(bar)
}
