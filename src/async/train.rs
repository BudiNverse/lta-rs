use reqwest::Error;
use tokio::prelude::Future;

use crate::r#async::lta_client::LTAClient;
use crate::train::train_service_alert::{TrainServiceAlert, TrainServiceAlertResp, URL};
use crate::utils::commons::Client;

pub fn get_train_service_alert(
    client: &LTAClient,
) -> impl Future<Item = TrainServiceAlert, Error = Error> {
    let rb = client.get_req_builder(URL);
    rb.send()
        .and_then(|mut r| r.json::<TrainServiceAlertResp>())
        .map(|r| r.value)
}