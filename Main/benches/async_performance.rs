use criterion::{criterion_group, criterion_main, Criterion};
use manufacturer::{Actions, sensing_data::Readings};
use async_lib::{Control, Sensing,transmission_control};
use std::sync::{atomic::AtomicI32, Arc};
use tokio::{runtime::Runtime, sync::Mutex};
fn measure_generation(c: &mut Criterion){
    let mut group= c.benchmark_group("Readings Generation Measurement");
    let scale=vec![10, 30, 50, 100];
    for size in scale.into_iter(){
        group.bench_function(format!("Readings generation, {:?} size",size), move |x|{
            x.iter(||{
                Readings::assign_data(size);
            });
        });

    }
}

fn measure_processing(c: &mut Criterion){
    let mut group= c.benchmark_group("Readings Processing");
    let scale=vec![10, 30, 50, 100];
    for size in scale.into_iter(){
        group.bench_function(format!("Readings Processing, {:?} size",size), move |x|{
            x.iter(||{
                Readings::assign_data(size).filter_noise();
            });
        });

    }
}

async fn execute_tramission(size : i32){
    let data=Readings::assign_data(size).filter_noise();
    let channel=transmission_control::TransmissionChannel::init();
    let counts=Arc::new(AtomicI32::new(data.objects_num.clone()));
    let sensing_info=Arc::new(Mutex::new((data.current_state, data.objects.clone())));
    data.sensor_control(sensing_info, channel.txes, channel.rxes, counts).await;
}

fn data_transmission(c: &mut Criterion){
    let mut group= c.benchmark_group("Data Transmission");
    let scale=vec![10, 30, 50, 100];
    for size in scale.into_iter(){
        let runtime= Runtime::new().expect("unable to create a tokio runtime");
        group.bench_function(format!("data transmission, {:?} size",size), move |x|
            x.to_async(&runtime).iter(async|| {
                execute_tramission(size).await;
            }),
        );
    }
}

criterion_group!(benches,measure_generation, measure_processing, data_transmission);
criterion_main!(benches);

