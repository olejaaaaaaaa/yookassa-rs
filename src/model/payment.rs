// Copyright (c) 2025 Oleg Pavlenko

use serde::{Deserialize, Serialize};
use super::*;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ResponsePayment {
    pub id: Option<String>,
    pub status: Option<String>,
    pub amount: Option<Amount>,
    pub income_amount: Option<IncomeAmount>,
    pub description: Option<String>,
    pub recipient: Option<Recipient>,
    pub payment_method: Option<PaymentMethod>,
    pub captured_at: Option<String>,
    pub created_at: Option<String>,
    pub test: Option<bool>,
    pub refunded_amount: Option<RefundedAmount>,
    pub paid: Option<bool>,
    pub refundable: Option<bool>,
    pub metadata: Option<serde_json::Value>
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ResponsePayments {
    pub r#type: String,
    pub next_cursor: String,
    pub items: Vec<ResponsePayment>
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct RequestCreatePayments {
    pub amount: Option<Amount>,
    pub description: Option<String>,
    pub confirmation: Option<Confirmation>,
    pub payment_method_data: Option<PaymentMethodData>
}

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

