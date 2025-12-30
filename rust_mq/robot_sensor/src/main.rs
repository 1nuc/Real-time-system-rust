use tokio::*;
use std::sync::Arc;
mod sensor;

#[allow(unused_variables)]
#[allow(non_snake_case)]
#[tokio::main]
async fn main() {
    let connection= Arc::new(sensor::create_connection().await);
    //create a communication channel
    let connection_cloned=Arc::clone(&connection);
    let channel=sensor::create_channel(connection_cloned).await;
    sensor::sensor_control(channel, connection).await;
    signal::ctrl_c().await.expect("failed");
}
