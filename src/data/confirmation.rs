// Copyright (c) 2025 Oleg Pavlenko

use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Confirmation {
    pub r#type: String,
    pub return_url: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ConfirmationResponse {
    confirmation_url: String,
}