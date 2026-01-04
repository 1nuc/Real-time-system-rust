use criterion::{criterion_group, criterion_main, Criterion};
use manufacturer::{Actions, sensing_data::Readings};
use sync_lib::{ControlSync, SensingSync,transmission_control};
use std::sync::{Mutex, atomic::AtomicI32, Arc};
fn measure_generation(c: &mut Criterion){
    let mut group= c.benchmark_group("Readings Generation Measurement");
    let scale=vec![10, 20, 30, 40, 50, 100];
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
    let scale=vec![10, 20, 30, 40, 50, 100];
    for size in scale.into_iter(){
        group.bench_function(format!("Readings Processing, {:?} size",size), move |x|{
            x.iter(||{
                Readings::assign_data(size).filter_noise();
            });
        });

    }
}

fn execute_tramission(size : i32){
    let data=Readings::assign_data(size).filter_noise();
    let channel=transmission_control::TransmissionChannel::init();
    let counts=Arc::new(AtomicI32::new(data.objects_num.clone()));
    let sensing_info=Arc::new(Mutex::new((data.current_state, data.objects.clone())));
    data.sensor_control(sensing_info, channel.txes, channel.rxes, counts);
}

fn data_transmission(c: &mut Criterion){
    let mut group= c.benchmark_group("Data Transmission");
    let scale=vec![10, 20, 30, 40, 50, 100];
    for size in scale.into_iter(){
        group.bench_function(format!("data transmission, {:?} size",size), move |x|{
            x.iter(||{
                execute_tramission(size);
            });
        });

    }
}

criterion_group!(benches,measure_generation, measure_processing, data_transmission);
criterion_main!(benches);

