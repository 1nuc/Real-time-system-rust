use criterion::{criterion_group, criterion_main, Criterion};
use tokio::runtime::Runtime;
use criterion::async_executor::Future;
use async_lib;
use sync_lib;

async fn async_benchmarker(){
    let ts_control=TransmissionCha
}
fn compare_async_sync(c: &mut Criterion){
    let mut group=c.benchmark_group("Async Vs Sync");
    let runtime=Runtime::new().expect("unable to create tokio runtime"); 
    group.bench_function("async", move |x|{
        x.to_async(Future).iter(||{

        });
        }
    );
}
criterion_group!();
criterion_main!(benches);
