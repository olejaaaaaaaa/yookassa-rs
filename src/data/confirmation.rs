// Copyright (c) 2025 Oleg Pavlenko

use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Confirmation {
    r#type: String,
    return_url: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ConfirmationResponse {
    confirmation_url: String,
}


/*

  "payment_method" : {
    "type" : "yoo_money",
    "id" : "2f8649e5-000f-5001-8000-1329b2b119fc",
    "saved" : false,
    "status" : "inactive",
    "title" : "YooMoney wallet 410011758831136",
    "account_number" : "410011758831136"
  },

*/