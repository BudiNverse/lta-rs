use crate::client_config::CLIENT_CONFIG;
use crate::utils::commons::build_res;

pub mod taxi_avail {
    use serde::Deserialize;

    pub const URL: &'static str =
        "http://datamall2.mytransport.sg/ltaodataservice/Taxi-Availability";

    #[derive(Debug, Clone, PartialEq, Deserialize)]
    pub struct TaxiPos {
        #[serde(rename = "Longitude")]
        pub long: f64,

        #[serde(rename = "Latitude")]
        pub lat: f64,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize)]
    pub struct TaxiAvailResp {
        pub value: Vec<TaxiPos>
    }
}


/// Returns location coordinates of all Taxis that are currently available for
/// hire. Does not include "Hired" or "Busy" Taxis.
///
/// Update freq: 1min
pub fn get_taxi_avail() -> reqwest::Result<Vec<taxi_avail::TaxiPos>> {
    let resp: taxi_avail::TaxiAvailResp = build_res(taxi_avail::URL)?;
    Ok(resp.value)
}
