use sync_lib::*;
// use async_lib::*;
// #[tokio::main]
fn main() {
        let simulation=transmission_control::TransmissionChannel::init();
        simulation.simulation_control(40);
        // let simulation=transmission_control::TransmissionChannel::init();
        // simulation.simulation_control(30).await;

}

