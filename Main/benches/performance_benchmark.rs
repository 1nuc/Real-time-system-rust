use criterion::{criterion_group, criterion_main, Criterion};
use tokio::runtime::Runtime;
use async_lib::{Control};
use sync_lib::{ControlSync};

async fn async_benchmarker(size: i32){
    let ts_control=async_lib::transmission_control::TransmissionChannel::init();
    ts_control.simulation_control(size).await;
}

fn sync_benchmarker(size: i32){
    let ts_control=sync_lib::transmission_control::TransmissionChannel::init();
    ts_control.simulation_control(size);
}

fn compare_async_sync(c: &mut Criterion){
    let mut group=c.benchmark_group("Async Vs Sync");
    let sizes=vec![10, 30, 50, 100];
    for size in sizes.into_iter(){
        let runtime=Runtime::new().expect("unable to create tokio runtime"); 
        group.bench_function(format!("async version using tokio for size {:?}", size), |x|x.to_async(&runtime).iter(||async{
                    async_benchmarker(size).await;
            }),
        );
        group.bench_function(format!("sync version using rust multi thread environment for size {:?}", size), |x|x.iter(||{
            sync_benchmarker(size);
        }));

    }
}
criterion_group!(benches, compare_async_sync);
criterion_main!(benches);
