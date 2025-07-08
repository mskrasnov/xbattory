//! `/sys/class/power_supply/BAT*/uevent` file parser

use anyhow::Result;
use std::{collections::HashMap, fmt::Display, fs::read_to_string, path::Path};

#[derive(Debug)]
pub struct Uevent {
    pub name: String,
    pub status: Status,
    pub technology: String,
    pub cycle_count: isize,
    pub voltage_min_design: usize,
    pub voltage_now: usize,
    pub power_now: usize,
    pub energy_full_design: usize,
    pub energy_full: usize,
    pub energy_now: usize,
    pub capacity: u8,
    pub capacity_level: Level,
    pub model_name: String,
    pub manufacturer: String,
    pub serial_number: String,
    pub health: f32,
}

impl Uevent {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = read_to_string(&path)?;
        let data = contents
            .split('\n')
            .map(|chunk| {
                let chunks = chunk
                    .split('=')
                    .map(|chunk| chunk.trim())
                    .collect::<Vec<_>>();
                if chunks.len() < 2 {
                    ("", "")
                } else {
                    (chunks[0], chunks[1])
                }
            })
            .collect::<HashMap<&str, &str>>();

        Ok(Self {
            name: data.get("POWER_SUPPLY_NAME").unwrap_or(&"").to_string(),
            status: Status::from(*data.get("POWER_SUPPLY_STATUS").unwrap_or(&"Unknown")),
            technology: data
                .get("POWER_SUPPLY_TECHNOLOGY")
                .unwrap_or(&"Unknown")
                .to_string(),
            cycle_count: data
                .get("POWER_SUPPLY_CYCLE_COUNT")
                .unwrap_or(&"0")
                .parse()?,
            voltage_min_design: data
                .get("POWER_SUPPLY_VOLTAGE_MIN_DESIGN")
                .unwrap_or(&"0")
                .parse()?,
            voltage_now: data
                .get("POWER_SUPPLY_VOLTAGE_NOW")
                .unwrap_or(&"0")
                .parse()?,
            power_now: data.get("POWER_SUPPLY_POWER_NOW").unwrap_or(&"0").parse()?,
            energy_full_design: data
                .get("POWER_SUPPLY_ENERGY_FULL_DESIGN")
                .unwrap_or(&"0")
                .parse()?,
            energy_full: data
                .get("POWER_SUPPLY_ENERGY_FULL")
                .unwrap_or(&"0")
                .parse()?,
            energy_now: data
                .get("POWER_SUPPLY_ENERGY_NOW")
                .unwrap_or(&"0")
                .parse()?,
            capacity: data.get("POWER_SUPPLY_CAPACITY").unwrap_or(&"0").parse()?,
            capacity_level: Level::from(
                *data.get("POWER_SUPPLY_CAPACITY_LEVEL").unwrap_or(&"Other"),
            ),
            model_name: data
                .get("POWER_SUPPLY_MODEL_NAME")
                .unwrap_or(&"")
                .to_string(),
            manufacturer: data
                .get("POWER_SUPPLY_MANUFACTURER")
                .unwrap_or(&"")
                .to_string(),
            serial_number: data
                .get("POWER_SUPPLY_SERIAL_NUMBER")
                .unwrap_or(&"")
                .to_string(),
            health: {
                (data
                    .get("POWER_SUPPLY_ENERGY_FULL")
                    .unwrap_or(&"0")
                    .parse::<f32>()?
                    / data
                        .get("POWER_SUPPLY_ENERGY_FULL_DESIGN")
                        .unwrap_or(&"0")
                        .parse::<f32>()?)
                    * 100.
            },
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Status {
    Full,
    Discharging,
    Charging,
    NotCharging,
    Unknown,
}

impl From<&str> for Status {
    fn from(value: &str) -> Self {
        match value {
            "Full" => Self::Full,
            "Discharging" => Self::Discharging,
            "Charging" => Self::Charging,
            _ => Self::Unknown,
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Full => "Full",
                Self::Discharging => "Discharging",
                Self::Charging => "Charging",
                Self::NotCharging => "Not charging",
                Self::Unknown => "???",
            }
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Level {
    Full,
    Normal,
    Low,
    Critical,
    Unknown,
}

impl From<&str> for Level {
    fn from(value: &str) -> Self {
        match value {
            "Full" => Self::Full,
            "Normal" => Self::Normal,
            _ => Self::Unknown,
        }
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Full => "Full",
                Self::Normal => "Normal",
                Self::Low => "Low",
                Self::Critical => "Critical",
                Self::Unknown => "???",
            }
        )
    }
}
