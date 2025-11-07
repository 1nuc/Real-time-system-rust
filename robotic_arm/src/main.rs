use source_lib::{actuator::*, sensor::*, transmission_control::*, Actions, Actuator, Control, Sensing};
use advanced_pid::Pid;
fn main() {
    let simulation=TransmissionChannel::init();
    simulation.simulation_control();
}
