use criterion::{criterion_group, criterion_main, Criterion};
use tokio::runtime::Runtime;
use criterion::async_executor::FuturesExecuter;
use async_lib::{Control};
use sync_lib::{Control, *};

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
    let runtime=Runtime::new().expect("unable to create tokio runtime"); 
    let size=50;
    group.bench_function("async version using tokio", move |x|{
        x.to_async(Future).iter(||async{
            async_benchmarker(size).await;
        });
        }
    );
    group.bench_function("sync version using rust multi thread environment", |x|x.iter(||{
        sync_benchmarker(size);
    }));
}
criterion_group!(benches, compare_async_sync);
criterion_main!(benches);
