// Copyright (c) 2025 Oleg Pavlenko and other contributors

use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Recipient {
    pub account_id: String,
    pub gateway_id: String,
}
