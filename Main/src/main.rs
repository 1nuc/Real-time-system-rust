// use sync_lib::*;
// use async_lib::*;
// use tokioo;
use tokio::{process::Command, try_join};
use std::path::Path;
#[tokio::main]
async fn main() {
        // let simulation=transmission_control::TransmissionChannel::init();
        // simulation.simulation_control(32);
        // let simulation=transmission_control::TransmissionChannel::init();
        // simulation.simulation_control(30).await;
    // let _delete_server_cache=Command::new("rm").args(&["-r", "../../rust_mq/robot_actuator/target/"]).output().await; 
    // let _delete_client_cache=Command::new("rm").args(&["-r", "../../rust_mq/robot_sensor/target/"]).output().await; 
    //
    // let _build_server=Command::new("cargo").args(&["b", "--manifest-path", "../../rust_mq/robot_sensor/Cargo.toml"]).output().await; 
    // let _build_client=Command::new("cargo").args(&["b", "--manifest-path", "../../rust_mq/robot_actuator/Cargo.toml"]).output().await; 

    let client="../../rust_mq/robot_actuator/target/debug/robot_actuator";
    let server="../../rust_mq/robot_sensor/target/debug/robot_sensor";
    let  mut server_t=Command::new(server).spawn().expect("error in server");
    let  mut client_t=Command::new(client).spawn().expect("error in the client"); 

    let _=try_join!(server_t.wait(), client_t.wait());

}

