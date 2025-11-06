use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use crate::{Control};

pub struct TransmissionChannel<T>{
    pub txes: Sender<T>,
    pub rxes: Receiver<T>,
}
impl <T, E> Control<T, E> for TransmissionChannel<T>{
    fn init()-> Self{
        let (tx, rx) = channel::<T>();
        Self{
            txes: tx,
            rxes: rx,
        }
    } 

    fn receive_packets(&self) -> Result<T, TryRecvError> {
        self.rxes.try_recv()
    }
    fn clone(&self) -> Sender<T>{
        self.txes.clone()
    }
}

