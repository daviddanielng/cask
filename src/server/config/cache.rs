use serde::Deserialize;

use crate::utils::{macros, memory, util};

#[derive(Clone, Deserialize, Debug)]
pub enum ServerCacheMode {
    Fill,
    Hit,
}
#[derive(Clone, Deserialize, Debug)]
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
    #[serde(
        default = "default_memory_check_interval",
        deserialize_with = "deserialize_memory_check_interval"
    )]
    pub memory_check_interval: u64,
    #[serde(
        default = "default_eviction_threshold",
        deserialize_with = "deserialize_eviction_threshold"
    )]
    pub eviction_threshold: u8,
}

fn default_eviction_threshold() -> u8 {
    90 // Default to evicting when memory usage reaches 90%
}
fn deserialize_eviction_threshold<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value_str = String::deserialize(deserializer)?;
    let value_str = value_str.trim_end_matches('%');
    let value: u8 = value_str.parse().map_err(|_| {
        serde::de::Error::custom(format!(
            "Invalid eviction-threshold value: {}. Must be a percentage (e.g., 90%).",
            value_str
        ))
    })?;
    if value == 0 || value > 100 {
        return Err(serde::de::Error::custom(
            "eviction-threshold must be a percentage between 1 and 100.",
        ));
    }
    Ok(value)
}
fn default_memory_check_interval() -> u64 {
    60 // Default to checking memory every 60 seconds
}
fn deserialize_memory_check_interval<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    if value.contains(".") {
        return Err(serde::de::Error::custom(format!(
            "Invalid memory-check-interval value: {}. Must be a number followed by an optional unit (S for seconds, M for minutes, H for hours).",
            value
        )));
    }
    let (value_str, unit) = value.chars().partition::<String, _>(|c| c.is_digit(10));
    let value: f64 = value_str.parse().map_err(|_| {
        serde::de::Error::custom(format!(
            "Invalid memory-check-interval value: {}. Must be a number followed by an optional unit (S for seconds, M for minutes, H for hours).",
            value
        ))
    })?;
    if value == 0.0 {
        return Err(serde::de::Error::custom(
            "memory-check-interval must be a positive integer.",
        ));
    }
    let multiplier = match unit.to_uppercase().as_str() {
        "S" => 1.0,
        "M" => 60.0,
        "H" => 60.0 * 60.0,
        "" => 1.0, // Default to seconds if no unit is provided
        _ => {
            return Err(serde::de::Error::custom(format!(
                "Invalid memory-check-interval unit: {}. Supported units: S (seconds), M (minutes), H (hours).",
                unit
            )));
        }
    };
    macros::log_verbose!(
        "Parsed memory-check-interval value: {} with unit: {} to {} seconds",
        value,
        unit,
        value * multiplier
    );
    Ok((value * multiplier) as u64)
}

fn default_mode() -> ServerCacheMode {
    ServerCacheMode::Hit
}
fn deserialize_cache_mode<'de, D>(deserializer: D) -> Result<ServerCacheMode, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let mode_str = String::deserialize(deserializer)?.to_lowercase();
    match mode_str.as_str() {
        "fill" => Ok(ServerCacheMode::Fill),
        "hit" => Ok(ServerCacheMode::Hit),
        _ => Err(serde::de::Error::custom(format!(
            "Invalid cache mode: {}. Supported modes: fill , hit",
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
    if value.contains(".") {
        return Err(serde::de::Error::custom(format!(
            "Invalid counter-reset value: {}. Must be a number followed by an optional unit (S for seconds, M for minutes, H for hours, D for days).",
            value
        )));
    }
    let (value_str, unit) = value.chars().partition::<String, _>(|c| c.is_digit(10));
    let value: f64 = value_str.parse().map_err(|_| {
        serde::de::Error::custom(format!(
            "Invalid counter-reset value: {}. Must be a number followed by an optional unit (S for seconds, M for minutes, H for hours, D for days).",
            value
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
    let free_memory = memory::free_memory();
    let total_memory = memory::total_memory();
    // use the smaller of a half of the free memory or a third of the total memory as a safe default
    let default_max = std::cmp::min(free_memory / 2, total_memory / 3);
    macros::log_verbose!(
        "Calculated default max-memory: {} (free memory: {}, total memory: {})",
        util::bytes_to_readable_size(default_max),
        util::bytes_to_readable_size(free_memory),
        util::bytes_to_readable_size(total_memory)
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
        memory::total_memory_with_format(Some(memory::MemoryFormat::Bytes));
    if value == 0.0 {
        return Err(serde::de::Error::custom(
            "max-memory must be a positive number.",
        ));
    }
    if value > total_memory as f64 {
        return Err(serde::de::Error::custom(format!(
            "max-memory value {} exceeds total system memory of {}.",
            util::bytes_to_readable_size(value as u64),
            util::bytes_to_readable_size(total_memory)
        )));
    }

    if value > (default_max_memory() as f64 * 1.3) {
        // only show warning if the value is 1.3 times higher than the calculated default to avoid unnecessary warnings for values that are reasonably close to the default
        macros::log_warning!(
            "Specified max-memory value {} is quite high compared to the calculated default of {}. Ensure that this is intentional and that your system has enough resources to handle it.",
            util::bytes_to_readable_size(value as u64),
            util::bytes_to_readable_size(default_max_memory())
        );
    }

    macros::log_verbose!(
        "Parsed max-memory value: {} with unit: {} to {} bytes",
        value,
        unit,
        value
    );
    Ok((value) as u64)
}
