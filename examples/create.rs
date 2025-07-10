


use uuid::Uuid;
use yookassa_rs::prelude::*;
use yookassa_rs::models::*;

#[tokio::main]
async fn main() {

    let shop_id = "1045131";
    let secret_key = "test_1HtqeKIGTQan9ODGjHdfIXTaTv1U3yTU0JDiqUbsGj4";

    let client = YookassaClientBuilder::new(
        BasicAuth::new(secret_key, shop_id))
        .build();

    let mut header = HeaderMap::new();
    header.insert("Idempotence-Key", HeaderValue::from_str(&Uuid::new_v4().to_string()).unwrap());
    header.insert("Content-Type", HeaderValue::from_str("application/json").unwrap());

    let resp = client
        .request(Method::POST, "/payments")
        .headers(header)
        .json(&RequestCreatePayments {
            amount: Amount {
                value: "100".into(),
                currency: "RUB".into(),
            },
            confirmation: Some(Confirmation {
                r#type: "redirect".into(),
                return_url: "https://www.exmaple.com".into()
            }),
            payment_method_data: Some(PaymentMethodData {
                r#type: "bank_card".into()
            }),
            description: Some("Pay Subscription".into()),
            ..Default::default()
        })
        .send::<serde_json::Value>()
        .await;

    println!("{:?}", resp);
    //assert_eq!(resp.is_ok(), true);

}


