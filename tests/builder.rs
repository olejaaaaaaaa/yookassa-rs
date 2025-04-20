// Copyright (c) 2025 Oleg Pavlenko

const SHOP_ID: &str = "1045131";
const SECRET_KEY: &str = "test_1HtqeKIGTQan9ODGjHdfIXTaTv1U3yTU0JDiqUbsGj4";

#[cfg(test)]
mod tests {
  
    use uuid::Uuid;
    use yookassa_rs::models::*;
    use yookassa_rs::prelude::*;

    use crate::SECRET_KEY;
    use crate::SHOP_ID;

    #[tokio::test]
    async fn with_custom_client() {

    }

    #[tokio::test]
    async fn get_paymant() {

    }

    #[tokio::test]
    async fn get_payments() {

    }

    #[tokio::test]
    async fn builder() {
        
    }

    #[tokio::test]
    async fn create_payment() {

        let client = YookassaClientBuilder::default()
            .auth(BasicAuth::new(SECRET_KEY, SHOP_ID))
            .build();
    
        let mut header = HeaderMap::new();
        header.insert("Idempotence-Key", HeaderValue::from_str(&Uuid::new_v4().to_string()).unwrap());
        header.insert("Content-Type", HeaderValue::from_str("application/json").unwrap());

        let id = "2f8649e5-000f-5001-8000-1329b2b119fc";
        let resp = client
            .request(Method::POST, "/payments")
            .headers(header)
            .json(&RquestCreatePayments {
                amount: Some(Amount {
                    value: "100".into(),
                    currency: "RUB".into(),
                }),
                confirmation: Some(Confirmation {
                    r#type: "redirect".into(),
                    return_url: "https://www.exmaple.com".into()
                }),
                payment_method_data: Some(PaymentMethodData {
                    r#type: "bank_card".into()
                }),
                description: Some("Опять новый платеж".into()),
                ..Default::default()
            })
            .send::<serde_json::Value>()
            .await;
    
        println!("{:?}", resp);
    }

}