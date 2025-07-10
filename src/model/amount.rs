use serde::*;


#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Amount {
    pub value: String,
    pub currency: String
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
pub struct IncomeAmount {
    value: String,
    currency: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
pub struct RefundedAmount {
    pub value: String,
    pub currency: String
}
