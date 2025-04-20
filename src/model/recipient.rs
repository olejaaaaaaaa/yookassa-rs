use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Recipient {
    pub account_id: String,
    pub gateway_id: String,
}
