use std::env;
use franklin_wh;

#[tokio::main]
async fn main() {
    let token = env::args().nth(1).unwrap();
    let gateway = env::args().nth(2).unwrap();
    let client = franklin_wh::Client::new(&token, &gateway);
    println!("SoC: {}", client.get_state_of_charge().await.unwrap());
    println!("inner: {:?}", client.get_iot_user_runtime_datalog().await.unwrap());
}
