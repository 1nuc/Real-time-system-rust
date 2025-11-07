use std::sync::mpsc::{channel, Receiver, Sender};
use crate::{sensor::{ReadingType, Readings}, Actuator, Control, Sensing};
use advanced_pid::{Pid};

pub struct TransmissionChannel{
    pub txes: Sender<ReadingType>,
    pub rxes: Receiver<ReadingType>,
}
impl Control for TransmissionChannel{
    fn init()-> Self{
        let (tx, rx) = channel::<ReadingType>();
        Self{
            txes: tx,
            rxes: rx,
        }
    } 

    fn clone(&self) -> Sender<ReadingType>{
        self.txes.clone()
    }

    fn simulation_control(self){
        let robotic_data=Readings::assign_data(30).filter_noise();
        robotic_data.transmit_data(self.txes.clone());
        Pid::recieve_transmission(self.rxes);
    }
}

