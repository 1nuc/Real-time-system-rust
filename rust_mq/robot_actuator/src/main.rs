use tokio::*;
mod actuator;
use std::sync::Arc;


#[allow(unused_variables)]
#[allow(non_snake_case)]
#[tokio::main]
async fn main() {
    let connection =Arc::new(actuator::create_connection().await);

    let connection_cloned=Arc::clone(&connection);
    actuator::actuator_control(connection_cloned).await;

    signal::ctrl_c().await.expect("unable to terminate");
}
