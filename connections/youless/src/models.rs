use chrono::{DateTime, Utc};
use prost::Message;
use serde::Deserialize;

#[derive(Deserialize, Message)]
#[must_use]
pub struct Settings {
    #[prost(string, tag = "1", required)]
    pub host: String,
}

/// YouLess API response ([Enelogic][1] firmware).
///
/// [1]: https://wiki.td-er.nl/index.php?title=YouLess#Enelogic_.28default.29_firmware
#[serde_with::serde_as]
#[derive(Deserialize)]
#[must_use]
pub struct Counters {
    #[serde(rename = "tm")]
    #[serde_as(as = "serde_with::TimestampSeconds<i64>")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "pwr")]
    pub actual_power_watt: f64,

    #[serde(rename = "p1")]
    pub electricity_consumption_low_kwh: f64,

    #[serde(rename = "p2")]
    pub electricity_consumption_high_kwh: f64,

    #[serde(rename = "n1")]
    pub electricity_production_low_kwh: f64,

    #[serde(rename = "n2")]
    pub electricity_production_high_kwh: f64,

    #[serde(rename = "net")]
    pub electricity_consumption_net_kwh: f64,

    #[serde(rename = "gas")]
    pub gas_consumption_m3: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_counters_ok() {
        // language=json
        let counters: Counters = serde_json::from_str(
            r#"{"tm":1666532025,"net": 11993.485,"pwr": 512,"ts0":1663603259,"cs0": 0.000,"ps0": 0,"p1": 9334.267,"p2": 8179.826,"n1": 1713.316,"n2": 3807.292,"gas": 7148.355,"gts":2210231530}"#,
        ).unwrap();
        assert_eq!(counters.actual_power_watt, 512.0);
        assert_eq!(counters.electricity_consumption_low_kwh, 9334.267);
    }
}
