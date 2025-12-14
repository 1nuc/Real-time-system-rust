use std::sync::{Arc, Mutex};
use crate::{Actuator, Control, Sensing, sensor::{ReadingType, Readings}};
use crossbeam::channel::*;
use advanced_pid::{Pid};

pub struct TransmissionChannel{
    pub txes: Sender<ReadingType>,
    pub rxes: Receiver<ReadingType>,
}
impl Control for TransmissionChannel{
    fn init()-> Self{
        let (tx, rx) = unbounded::<ReadingType>();
        Self{
            txes: tx,
            rxes: rx,
        }
    } 

    fn simulation_control(self){
        let robotic_data=Readings::assign_data(30).filter_noise();
        let sensing_info= Arc::new(Mutex::new((robotic_data.current_state, robotic_data.objects.clone())));
        println!("objects :{}", robotic_data.objects_num);
        let feedback_channel=Self::init();
        robotic_data.sensor_control(Arc::clone(&sensing_info),self.txes.clone(), feedback_channel.rxes);
        Pid::actuator_control(Arc::clone(&sensing_info),self.rxes, robotic_data.objects_num, feedback_channel.txes);
    }
}

