use serde::Deserialize;

use crate::utils::{logger, macros, memory, util};

#[derive(Clone, Deserialize)]
pub enum ServerCacheMode {
    Fill,
    Hits,
}
#[derive(Clone, Deserialize)]
pub struct ServerCache {
    #[serde(
        default = "default_counter_reset",
        deserialize_with = "deserialize_counter_reset"
    )]
    pub counter_reset: u64,
    #[serde(default = "default_mode", deserialize_with = "deserialize_cache_mode")]
    pub mode: ServerCacheMode,
    #[serde(
        default = "default_max_memory",
        deserialize_with = "deserialize_max_memory"
    )]
    pub max_memory: u64,
}

fn default_mode() -> ServerCacheMode {
    ServerCacheMode::Hits
}
fn deserialize_cache_mode<'de, D>(deserializer: D) -> Result<ServerCacheMode, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let mode_str = String::deserialize(deserializer)?.to_lowercase();
    match mode_str.as_str() {
        "fill" => Ok(ServerCacheMode::Fill),
        _ => Err(serde::de::Error::custom(format!(
            "Invalid cache mode: {}. Supported modes: fill",
            mode_str
        ))),
    }
}
fn default_counter_reset() -> u64 {
    // Default to six hours
    60 * 60 * 6
}
fn deserialize_counter_reset<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    let (value_str, unit) = value
        .chars()
        .partition::<String, _>(|c| c.is_digit(10) || *c == '.');
    let value: f64 = value_str.parse().map_err(|_| {
        serde::de::Error::custom(format!(
            "Invalid counter-reset value: {}. Must be a number followed by an optional unit (S for seconds, M for minutes, H for hours, D for days).",
            value_str
        ))
    })?;
    if value == 0.0 {
        return Err(serde::de::Error::custom(
            "counter-reset must be a positive integer.",
        ));
    }
    let multiplier = match unit.to_uppercase().as_str() {
        "S" => 1.0,
        "M" => 60.0,
        "H" => 60.0 * 60.0,
        "D" => 60.0 * 60.0 * 24.0,
        "" => 1.0, // Default to seconds if no unit is provided
        _ => {
            return Err(serde::de::Error::custom(format!(
                "Invalid counter-reset unit: {}. Supported units: S (seconds), M (minutes), H (hours), D (days).",
                unit
            )));
        }
    };
    macros::log_verbose!(
        "Parsed counter-reset value: {} with unit: {} to {} seconds",
        value,
        unit,
        value * multiplier
    );
    Ok((value * multiplier) as u64)
}
fn default_max_memory() -> u64 {
    let free_memory = crate::utils::memory::free_memory_with_format(Some(memory::MemoryFormat::MB));
    let total_memory =
        crate::utils::memory::total_memory_with_format(Some(memory::MemoryFormat::MB));
    // use the smaller of a third of the free memory or a quarter of the total memory as a safe default
    let default_max = std::cmp::min(free_memory / 3, total_memory / 4);
    macros::log_verbose!(
        "Calculated default max-memory: {} MB (free memory: {} MB, total memory: {} MB)",
        default_max,
        free_memory,
        total_memory
    );
    default_max
}

fn deserialize_max_memory<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let mem_str = String::deserialize(deserializer)?.to_uppercase();
    let (value_str, unit) = mem_str
        .chars()
        .partition::<String, _>(|c| c.is_digit(10) || *c == '.');
    let value: f64 = value_str.parse().map_err(|_| {
        serde::de::Error::custom(format!(
            "Invalid memory value: {}. Must be a number followed by an optional unit (B, KB, MB, GB).",
            value_str
        ))
    })?;
    let multiplier = match unit.as_str() {
        "B" => 1.0,
        "KB" => 1024.0,
        "MB" => 1024.0 * 1024.0,
        "GB" => 1024.0 * 1024.0 * 1024.0,
        _ => {
            return Err(serde::de::Error::custom(format!(
                "Invalid memory unit: {}. Supported units: B, KB, MB, GB.",
                unit
            )));
        }
    };
    let value = value * multiplier;
    let total_memory =
        crate::utils::memory::total_memory_with_format(Some(memory::MemoryFormat::Bytes));
    if value > total_memory as f64 {
        return Err(serde::de::Error::custom(format!(
            "max-memory value {} exceeds total system memory of {}.",
            util::bytes_to_readable_size(value as u64),
            util::bytes_to_readable_size(total_memory)
        )));
    }
    macros::log_verbose!(
        "Parsed max-memory value: {} with unit: {} to {} bytes",
        value,
        unit,
        value * multiplier
    );
    Ok((value * multiplier) as u64)
}
