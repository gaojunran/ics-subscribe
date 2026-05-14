use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct EventConfig {
    pub title: String,
    pub start_time: String,
    pub end_time: Option<String>,
    pub location: Option<String>,
    pub description: Option<String>,
    pub rrule: Option<String>,
    /// Reminder minutes before event start, e.g. 15
    pub reminder: Option<i64>,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub calendar_name: Option<String>,
    pub events: Vec<EventConfig>,
}

impl Config {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
