use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use crate::{sensor::Readings, Actuator, Control, Sensing};
use advanced_pid::{Pid};

pub struct TransmissionChannel<T>{
    pub txes: Sender<T>,
    pub rxes: Receiver<T>,
}
impl <T> Control<T > for TransmissionChannel<T>{
    fn init()-> Self{
        let (tx, rx) = channel::<T>();
        Self{
            txes: tx,
            rxes: rx,
        }
    } 

    fn clone(&self) -> Sender<T>{
        self.txes.clone()
    }

    fn simulation_control(){
        let robotic_data=Readings::assign_data(30).filter_noise();
        let sensing_channel=TransmissionChannel::init();
        robotic_data.transmit_data(&sensing_channel.txes);
        Pid::recieve_transmission(&sensing_channel.rxes);
    }
}

