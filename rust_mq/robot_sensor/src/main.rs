use tokio::*;
use lapin::Connection;
mod sensor;

#[allow(unused_variables)]
#[allow(non_snake_case)]
#[tokio::main]
async fn main() {
    
    
    let connection: Connection= sensor::create_connection().await;
    //create a communication channel
    let channel=sensor::create_channel(connection).await;
    sensor::send(channel).await;
    signal::ctrl_c().await.expect("failed");
}
