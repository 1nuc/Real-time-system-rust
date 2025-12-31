use std::{time::Instant,sync::{Arc, atomic::{AtomicI32},MutexGuard}};
use crossbeam::{channel::*};
use manufacturer::sensing_data::{Actual, Target, Readings};
use crate::sensor::{ReadingType};
pub mod sensor;
pub mod actuator;
pub mod transmission_control;

pub trait Shared{
    type SharedLock<'a>;
    type Type;
}
pub trait ActuatorSync <'a>: Shared{
    fn actuator_control(sensing_info: Self::Type,sensor_recv: Receiver<ReadingType>, counts: Arc<AtomicI32>, feedback_send: Sender<ReadingType>, robotic_data: Readings);
    fn process_singals(lock: Self::SharedLock<'a>, current_arm_status: Actual,
        object_status: Target, id: i32, feedback_send: Sender<ReadingType>, robotic_data: Readings, counts: Arc<AtomicI32>);
    fn process_feedback(pos: f32, temparture: f32, force: f32, vel: f32, id: i32, robotic_data: Readings, feedback_send: Sender<ReadingType>, counts: Arc<AtomicI32>);
}
pub trait SensingSync: Shared{
    fn collect_data(&self, sensing_info: Self::Type,sensor_send: Sender<ReadingType>, counts: Arc<AtomicI32>);
    fn sensor_control(&self, sensing_info: Self::Type,sensor_send: Sender<ReadingType>, feedback_recv: Receiver<ReadingType>, counts: Arc<AtomicI32>);
    fn sensor_workflow(packets: Self::Type, tx_copy: Sender<ReadingType>);
}
pub trait ControlSync: Shared{
    fn init()-> Self;
    fn simulation_control(self, boxes_num: i32);
    fn transmit_data<'a>(data: ReadingType, sensor_send: Sender<ReadingType>, lock: Self::SharedLock<'a>);
    fn recieve_transmission(sensor_recv: Receiver<ReadingType>)->Option<ReadingType>;
    fn recieve_transmission_deadline(now: Instant,object_lock:MutexGuard<Vec<(Target, String, i32)>>, arm_status: Arc<Actual>,feedback_recv: Receiver<ReadingType>);
} 
