
use yookassa_rs::{payment::Payment, prelude::*};
use reqwest::Method;

#[tokio::main]
async fn main() {

    let shop_id = "1045131";
    let secret_key = "test_1HtqeKIGTQan9ODGjHdfIXTaTv1U3yTU0JDiqUbsGj4";

    let client = YookassaClientBuilder::default()
        .auth(BasicAuth::new(secret_key, shop_id))
        .build();

    let resp = client
        .request(Method::GET, "/payments/5463")
        .send::<Payment>()
        .await;

    println!("{:?}", resp);
    
}