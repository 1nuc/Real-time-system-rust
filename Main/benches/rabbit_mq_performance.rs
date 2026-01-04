use criterion::{Criterion, criterion_group, criterion_main};
use tokio::{process::Command, runtime::Runtime, try_join};
use std::path::Path;

async fn build_paths(){
    let server_b_path="../../../rust_mq/robot_sensor";
    let client_b_path="../../../rust_mq/robot_actuator";

    if !Path::new(&format!("{}/target/", server_b_path)).is_dir(){
        let _build_server=Command::new("cargo").args(&["b", "--manifest-path", 
            &format!("{}/Cargo.toml",server_b_path)]).output().await; 
    }
    if !Path::new(&format!("{}/target/", client_b_path)).is_dir(){
        let _build_client=Command::new("cargo").args(&["b", "--manifest-path", 
            &format!("{}/Cargo.toml", client_b_path)]).output().await; 
    }
}

async fn run_server(){
    build_paths().await;
    let client="../rust_mq/robot_actuator/target/debug/robot_actuator";
    let server="../rust_mq/robot_sensor/target/debug/robot_sensor";

    let  mut server_t=Command::new(server).spawn().expect("error in server");
    let  mut client_t=Command::new(client).spawn().expect("error in the client"); 
    let _=try_join!(server_t.wait(), client_t.wait());
}

fn rabbit_mq(c: &mut Criterion){
    let runtime=Runtime::new().expect("unable to create tokio runtime"); 
    c.bench_function("Rabbit MQ --server", |x|x.to_async(&runtime).iter(||async{
                run_server().await;
        }),
    );

}

criterion_group!(benches,rabbit_mq);
criterion_main!(benches);
