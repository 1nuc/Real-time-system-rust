use std::{sync::{mpsc::channel, Arc, Mutex}, thread, time::Duration};

use source_lib::{actuator::*, sensor::*, Actions, PidExtended, Sensing};
use advanced_pid::Pid;
fn main() {
    let simulation= Readings::assign_data(10);
    let packets = simulation.filter_noise();
    let (tx, tr) =channel::<Readings>();
    let data= Arc::new(Mutex::new(packets));
    let data_copy=Arc::clone(&data);
    thread::spawn(move|| {
        let pkt=data_copy.lock().unwrap();
        tx.send(pkt.clone()).unwrap();
        thread::sleep(Duration::from_millis(100));
    });
}
