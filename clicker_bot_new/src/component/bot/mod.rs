use crate::component::{bar, bar::Bar};
use crate::traits::ToYaml;
use color_eyre::eyre::Report;
use device_query::{DeviceQuery, DeviceState, Keycode};
use serde::{Deserialize, Serialize};
use std::fs::{write, File};
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
pub struct Bot {
    name: String,
    trainer_health: Bar,
    trainer_soul: Bar,
    digimon_health: Bar,
    digimon_soul: Bar,
}

impl Default for Bot {
    fn default() -> Self {
        Bot {
            name: String::from("default"),
            trainer_health: Bar::default(),
            trainer_soul: Bar::default(),
            digimon_health: Bar::default(),
            digimon_soul: Bar::default(),
        }
    }
}

impl ToYaml for Bot {}

impl Bot {
    pub fn new() -> Result<Self, Report> {
        let bot = Bot {
            name: String::from("default"),
            trainer_health: bar::select("trainer_health")?,
            trainer_soul: bar::select("trainer_soul")?,
            digimon_health: bar::select("digimon_health")?,
            digimon_soul: bar::select("digimon_soul")?,
        };

        Ok(bot)
    }

    pub fn from_config(config: &PathBuf) -> Result<Self, Report> {
        let reader = File::open(config)?;
        let bot: Bot = serde_yaml::from_reader(reader).expect("Failed to create bot from config");
        Ok(bot)
    }

    pub fn start(&self) -> Result<(), Report> {
        let _wait = Duration::from_millis(100);

        let device_state = DeviceState::new();
        let mut keys = device_state.get_keys();
        let mut escape = keys.contains(&Keycode::Escape);

        while !escape {
            keys = device_state.get_keys();
            escape = keys.contains(&Keycode::Escape);
        }

        Ok(())
    }

    pub fn export(&mut self, output: &PathBuf) -> Result<(), Report> {
        // This is ridiculous
        self.name = output.file_stem().unwrap().to_str().unwrap().to_string();
        write(output, self.to_yaml())?;
        Ok(())
    }
}
