//! All APIs pertaining to buses

use crate::lta_client::{AsyncLTAClient, LTAClient};
use crate::utils::{build_req_async_with_query, build_req_async_with_skip};
use crate::utils::{BusRequests};
use crate::{api_url, base_url};
use async_trait::async_trait;
use lta_models::bus::bus_arrival::BusArrivalResp;
use lta_models::bus::bus_routes::BusRoute;
use lta_models::bus::bus_services::{BusService, BusServiceResp};
use lta_models::bus::bus_stops::BusStop;
use lta_models::bus::{bus_arrival, bus_routes, bus_services, bus_stops};
use lta_utils_commons::{reqwest, LTAResult};

pub struct Bus;

#[async_trait]
impl BusRequests for Bus {
    type ClientType = AsyncLTAClient;

    async fn get_arrival(
        c: &Self::ClientType,
        bus_stop_code: u32,
        service_no: Option<&str>,
    ) -> LTAResult<BusArrivalResp> {
        build_req_async_with_query::<bus_arrival::RawBusArrivalResp, _, _, _>(
            c,
            api_url!("/BusArrivalv2"),
            move |rb| match service_no {
                Some(srv_no) => rb.query(&[
                    ("BusStopCode", bus_stop_code.to_string()),
                    ("ServiceNo", srv_no.to_string()),
                ]),
                None => rb.query(&[("BusStopCode", bus_stop_code.to_string())]),
            },
        )
        .await
    }

    async fn get_bus_services(
        c: &Self::ClientType,
        skip: Option<u32>,
    ) -> LTAResult<Vec<BusService>> {
        build_req_async_with_skip::<bus_services::BusServiceResp, _, _>(c, api_url!("/BusServices"), skip)
            .await
    }

    async fn get_bus_routes(c: &Self::ClientType, skip: Option<u32>) -> LTAResult<Vec<BusRoute>> {
        build_req_async_with_skip::<bus_routes::BusRouteResp, _, _>(c, api_url!("/BusRoutes"), skip).await
    }

    async fn get_bus_stops(c: &Self::ClientType, skip: Option<u32>) -> LTAResult<Vec<BusStop>> {
        build_req_async_with_skip::<bus_stops::BusStopsResp, _, _>(c, api_url!("/BusStops"), skip).await
    }
}
