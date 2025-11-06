use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use crate::{TransmissionControl};

pub struct TransmissionChannel<T>{
    txes: Sender<T>,
    rxes: Receiver<T>,
}
impl <T, E> TransmissionControl<T, E> for TransmissionChannel<T>{
    fn new()-> Self{
        let (tx, rx) = channel();
        Self{
            txes: tx,
            rxes: rx,
        }
    } 

    fn send_packets(&self,packets: T){
       self.txes.send(packets).unwrap(); //optional for now unwrap will be deleted in the future
                                         //and replaced by expect 
    }
    fn receive_packets(&self) -> Result<T, TryRecvError> {
        self.rxes.try_recv()
    }
}

