use std::sync::{mpsc::{channel, Receiver, Sender}, Arc, Mutex};
use crate::{Actuator, Control, Sensing, sensor::{ReadingType, Readings, Target}};
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
        let sensing_info= Arc::new(Mutex::new((robotic_data.current_state, robotic_data.objects.clone())));
        println!("objects :{}", robotic_data.objects_num);
        let feedback_channel=Self::init();
        robotic_data.transmit_data(Arc::clone(&sensing_info),self.txes.clone(), feedback_channel.rxes);
        Pid::recieve_transmission(Arc::clone(&sensing_info),self.rxes, robotic_data.objects_num, feedback_channel.txes);
    }
}

