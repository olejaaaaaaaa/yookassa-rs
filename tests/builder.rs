// Copyright (c) 2025 Oleg Pavlenko

#[cfg(test)]
mod tests {
  
    use reqwest::Method;
    use yookassa_rs::prelude::*;
    use yookassa_rs::payment::Payment;

    #[tokio::test]
    async fn builder() {

        let shop_id = "1045131";
        let secret_key = "test_1HtqeKIGTQan9ODGjHdfIXTaTv1U3yTU0JDiqUbsGj4";

        let client = YookassaClientBuilder::default()
            .auth(BasicAuth::new(secret_key, shop_id))
            .build();
    
        let resp = client
            .request(Method::GET, "/payments/2f8649e5-000f-5001-8000-1329b2b119fc")
            .send::<Payment>()
            .await;
    
        assert_eq!(resp.is_ok(), true);
        println!("{:?}", resp);

    }
}