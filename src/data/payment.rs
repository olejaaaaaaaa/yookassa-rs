// Copyright (c) 2025 Oleg Pavlenko

use serde::{Deserialize, Serialize};
use crate::data::prelude::*;

/// Payment object yookassa see docs: https://yookassa.ru/developers/api
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ResponsePayment {
    id: Option<String>,
    status: Option<String>,
    amount: Option<Amount>,
    income_amount: Option<IncomeAmount>,
    description: Option<String>,
    recipient: Option<Recipient>,
    payment_method: Option<PaymentMethod>,
    captured_at: Option<String>,
    created_at: Option<String>,
    test: Option<bool>,
    refunded_amount: Option<RefundedAmount>,
    paid: Option<bool>,
    refundable: Option<bool>,
    metadata: Option<serde_json::Value>
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ResponsePayments {
    r#type: String,
    next_cursor: String,
    items: Vec<ResponsePayment>
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct RquestCreatePayments {
    pub amount: Option<Amount>,
    pub description: Option<String>,
    pub confirmation: Option<Confirmation>,
    pub payment_method_data: Option<PaymentMethodData>
}

/*

  "type" : "list",
  "next_cursor" : "2f6b5a75-000f-5000-9000-1f18be9c423c",
*/


/// PaymentMethod object yookassa see docs: https://yookassa.ru/developers/api
#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
pub struct PaymentMethod {
    pub r#type: String,
    pub id: String,
    pub saved: bool,
    pub title: String,
    pub account_number: String
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
pub struct PaymentMethodData {
    pub r#type: String,
}

