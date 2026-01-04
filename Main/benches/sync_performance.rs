use criterion::{criterion_group, criterion_main, Criterion};
use manufacturer::{Actions, sensing_data::Readings};

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

fn data_transmission(c: Criterion){

}

criterion_group!(benches,measure_generation, measure_processing);
criterion_main!(benches);

