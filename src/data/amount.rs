use serde::Deserialize;


#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Amount {
    pub value: String,
    pub currency: String
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct IncomeAmount {
    value: String,
    currency: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct RefundedAmount {
    pub value: String,
    pub currency: String
}
