use async_lib::{transmission_control::*, Control};
use std::hint::black_box;
use bma_benchmark::benchmark;
fn main() {
    // benchmark!(1000, {
        let simulation=TransmissionChannel::init();
        simulation.simulation_control();
    // });
}
