// Copyright (c) 2025 Oleg Pavlenko

use serde::{Deserialize, Serialize};
use super::*;

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

