
use yookassa_rs::prelude::*;
use yookassa_rs::models::*;

#[tokio::main]
async fn main() {

    let shop_id = "1045131";
    let secret_key = "test_1HtqeKIGTQan9ODGjHdfIXTaTv1U3yTU0JDiqUbsGj4";

    let client: YookassaClient<BasicAuth> = YookassaClientBuilder::new(
        BasicAuth::new(secret_key, shop_id))
        .build();

    let resp = client
        .request(Method::GET, "/payments")
        .send::<ResponsePayments>()
        .await;

    println!("{:?}", resp);

}