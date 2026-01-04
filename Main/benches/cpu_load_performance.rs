use criterion::{Criterion, criterion_group, criterion_main};
use tokio::runtime::Runtime;
use async_lib::{Control};
use sync_lib::{ControlSync};
use std::{hint::black_box,thread};
async fn async_benchmarker(size: i32){
    let ts_control=async_lib::transmission_control::TransmissionChannel::init();
    ts_control.simulation_control(size).await;
}

fn sync_benchmarker(size: i32){
    let ts_control=sync_lib::transmission_control::TransmissionChannel::init();
    ts_control.simulation_control(size);
}

fn cpu_load_sync(){
    let mut arr=vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
     while let Some(mut i) =arr.pop(){
        thread::spawn(move ||{
            println!("popped value, {}", i);
            loop{
               i+=1; 
               black_box(i);
               thread::sleep(std::time::Duration::from_micros(1));
            }
        });
    }
}
async fn cpu_load_async(){
    let mut arr=vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
    while let Some(mut i) =arr.pop(){
        tokio::task::spawn(async move {
            println!("popped value :{}", i);
            loop{
               i+=1; 
               black_box(i);
               let _=tokio::time::sleep(tokio::time::Duration::from_micros(1));
            }
        });
    }
}

fn async_load(c: &mut Criterion){
    let runtime=Runtime::new().expect("unable to create tokio runtime"); 
    runtime.block_on(async{
        cpu_load_async().await;
    });
    c.bench_function("async version with cpu load", |x|x.to_async(&runtime).iter(||async{
                async_benchmarker(50).await;
        }),
    );
}

fn sync_load(c: &mut Criterion){
    cpu_load_sync();
    c.bench_function("sync version with cpu load ", |x|x.iter(||{
        sync_benchmarker(50);
    }));

}
criterion_group!(benches, async_load, sync_load);
criterion_main!(benches);
