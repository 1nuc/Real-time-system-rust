use sync_lib;
use async_lib::*;
use tokio;
use std::hint::black_box;
use bma_benchmark::benchmark;
#[tokio::main]
async fn main() {
    // benchmark!(1000, {
        // let simulation=sync_lib::transmission_control::TransmissionChannel::init();
        // simulation.simulation_control();
    // });
    let simulation=transmission_control::TransmissionChannel::init();
    simulation.simulation_control().await;
    
}

