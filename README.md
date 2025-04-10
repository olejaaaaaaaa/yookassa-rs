
# Yookassa-rs
Simple wrapper around yookassa api

Usage:
```
use yookassa_rs::prelude::{payments::PaymentResponse, *};
use reqwest::Method;

#[tokio::main]
async fn main() {

    let shop_id = "1045131";
    let secret_key = "test_1HtqeKIGTQan9ODGjHdfIXTaTv1U3yTU0JDiqUbsGj4";

    let client = YookassaClientBuilder::default()
        .auth(BasicAuth::new(secret_key, shop_id))
        .build();

    let resp = client
        .request(Method::GET, "/payments")
        .send::<PaymentsResponse>()
        .await;

    println!("{:?}", resp);
}
```

How to add to own project?
```
[dependencies]
yookassa_rs = { git = "https://github.com/olejaaaaaaaa/yookassa-rs" }
```



## License

[MIT](https://choosealicense.com/licenses/mit/)
[Apache](https://choosealicense.com/licenses/apache/)

## Documentation

not ready yet

