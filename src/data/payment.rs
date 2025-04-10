// Copyright (c) 2025 Oleg Pavlenko

use serde::Deserialize;
use crate::data::prelude::*;

/// Payment object yookassa see docs: https://yookassa.ru/developers/api
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Payment {
    id: String,
    status: String,
    amount: Amount,
    income_amount: IncomeAmount,
    description: String,
    recipient: Recipient,
    payment_method: PaymentMethod,
    captured_at: String,
    created_at: String,
    test: bool,
    refunded_amount: RefundedAmount,
    paid: bool,
    refundable: bool,
    metadata: serde_json::Value
}

/// PaymentMethod object yookassa see docs: https://yookassa.ru/developers/api
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct PaymentMethod {
    r#type: String,
    id: String,
    saved: bool,
    title: String,
    account_number: String
}

