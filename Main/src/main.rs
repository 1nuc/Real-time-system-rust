// use sync_lib::*;
use async_lib::*;
use tokio;
#[tokio::main]
async fn main() {
        // let simulation=transmission_control::TransmissionChannel::init();
        // simulation.simulation_control(32);
        let simulation=transmission_control::TransmissionChannel::init();
        simulation.simulation_control(30).await;

}

