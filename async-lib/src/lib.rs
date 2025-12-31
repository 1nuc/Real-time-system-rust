use std::{time::Instant,sync::{Arc, atomic::{AtomicI32}}};
use tokio::sync::MutexGuard;
use flume::*;
use manufacturer::sensing_data::{Actual, Target, Readings};
use crate::sensor::{ReadingType};
pub mod sensor;
pub mod actuator;
pub mod transmission_control;

pub trait Shared{
    type SharedLock<'a>;
    type Type;
}
#[trait_variant::make(Send)]
pub trait Actuator <'a>: Shared{
    async fn actuator_control(sensing_info: Self::Type,sensor_recv: Receiver<ReadingType>, counts: Arc<AtomicI32>, feedback_send: Sender<ReadingType>, robotic_data: Readings);
    async fn process_singals(lock: Self::SharedLock<'a>, current_arm_status: Actual,
        object_status: Target, id: i32, feedback_send: Sender<ReadingType>, robotic_data: Readings, counts: Arc<AtomicI32>);
    async fn process_feedback(pos: f32, temparture: f32, force: f32, vel: f32, id: i32, robotic_data: Readings, feedback_send: Sender<ReadingType>, counts: Arc<AtomicI32>);
}
#[trait_variant::make(Send)]
pub trait Sensing: Shared{
    async fn collect_data(&self, sensing_info: Self::Type,sensor_send: Sender<ReadingType>, counts: Arc<AtomicI32>);
    async fn sensor_control(&self, sensing_info: Self::Type,sensor_send: Sender<ReadingType>, feedback_recv: Receiver<ReadingType>, counts: Arc<AtomicI32>);
    async fn sensor_workflow(packets: Self::Type, tx_copy: Sender<ReadingType>);
}
#[trait_variant::make(Send)]
pub trait Control: Shared{
    fn init()-> Self;
    async fn simulation_control(self, boxes_num: i32);
    async fn transmit_data<'a>(data: ReadingType, sensor_send: Sender<ReadingType>, lock: Self::SharedLock<'a>);
    async fn recieve_transmission(sensor_recv: Receiver<ReadingType>)->Option<ReadingType>;
    async fn recieve_transmission_deadline<'a>(now: Instant,object_lock:MutexGuard<'a,Vec<(Target, String, i32)>>, arm_status: Arc<Actual>,feedback_recv: Receiver<ReadingType>);
} 
