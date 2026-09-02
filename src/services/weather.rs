//! Weather service.
//!
//! Fetches current conditions from the free [wttr.in] API, which needs no key.
//! Values are gathered best-effort on a background thread and surfaced to the
//! shell via an [`Event::WeatherFetched`].
//!
//! [wttr.in]: https://wttr.in
//! [`Event::WeatherFetched`]: crate::events::Event::WeatherFetched

use serde::Deserialize;

/// Current conditions snapshot for the dashboard.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Weather {
    pub temp_c: Option<i32>,
    pub feels_c: Option<i32>,
    pub humidity: Option<i32>,
    pub wind_kmh: Option<i32>,
    pub condition: String,
    pub city: String,
}

fn inner() -> Result<Weather, String> {
    let out = std::process::Command::new("curl")
        .args([
            "-s",
            "--max-time",
            "6",
            "-A",
            "curl",
            "https://wttr.in/?format=j1",
        ])
        .output()
        .map_err(|e| e.to_string())?;

    if !out.status.success() {
        return Err(format!("curl exited with {}", out.status));
    }

    let text = String::from_utf8_lossy(&out.stdout);
    let root: Wttr = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    let current = root
        .current_condition
        .and_then(|v| v.into_iter().next());

    let mut w = Weather {
        condition: current
            .as_ref()
            .and_then(|c| c.weather_desc.as_ref())
            .and_then(|d| d.first())
            .map(|d| d.value.clone())
            .unwrap_or_default(),
        ..Default::default()
    };

    if let Some(c) = current {
        w.temp_c = parse_i32(c.temp_c.as_deref());
        w.feels_c = parse_i32(c.feels_like_c.as_deref());
        w.humidity = parse_i32(c.humidity.as_deref());
        w.wind_kmh = c
            .windspeed_kmph
            .as_deref()
            .and_then(|s| s.trim().parse::<f32>().ok())
            .map(|v| v.round() as i32);
    }

    if let Some(area) = root.nearest_area.and_then(|v| v.into_iter().next()) {
        if let Some(names) = area.area_name {
            if let Some(n) = names.into_iter().next() {
                w.city = n.value;
            }
        }
    }

    Ok(w)
}

/// Fetch the current weather, defaulting to a placeholder on any error so the
/// caller never has to deal with failures.
pub fn fetch() -> Weather {
    inner().unwrap_or_else(|_| Weather {
        condition: "Unavailable".to_string(),
        city: String::new(),
        temp_c: None,
        feels_c: None,
        humidity: None,
        wind_kmh: None,
    })
}

fn parse_i32(s: Option<&str>) -> Option<i32> {
    s.and_then(|v| v.trim().parse::<i32>().ok())
}

#[derive(Debug, Deserialize)]
struct Wttr {
    #[serde(default)]
    current_condition: Option<Vec<CurrentCondition>>,
    #[serde(default)]
    nearest_area: Option<Vec<NearestArea>>,
}

#[derive(Debug, Deserialize)]
struct CurrentCondition {
    #[serde(rename = "temp_C")]
    temp_c: Option<String>,
    #[serde(rename = "FeelsLikeC")]
    feels_like_c: Option<String>,
    #[serde(rename = "humidity")]
    humidity: Option<String>,
    #[serde(rename = "windspeedKmph")]
    windspeed_kmph: Option<String>,
    #[serde(rename = "weatherDesc")]
    weather_desc: Option<Vec<Desc>>,
}

#[derive(Debug, Deserialize)]
struct Desc {
    value: String,
}

#[derive(Debug, Deserialize)]
struct NearestArea {
    #[serde(rename = "areaName")]
    area_name: Option<Vec<Desc>>,
}
