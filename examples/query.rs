use std::env;
use franklin_wh;

#[tokio::main]
async fn main() {
    let token = env::args().nth(1).unwrap();
    let gateway = env::args().nth(2).unwrap();
    let client = franklin_wh::Client::new(&token, &gateway);
    let inner = client.get_iot_user_runtime_datalog().await.unwrap().inner().unwrap();

    println!(r#"
Status:

charge: {}

battery: {}
home_load: {}
solar: {}
"#,
    inner.state_of_charge(),
    inner.current_battery_draw() * -1.0,
    inner.current_home_load(),
    inner.current_solar_yield()
    )


}
