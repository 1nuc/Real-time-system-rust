use crate::{Sensing, sensor::{ReadingType}, transmission_control::TransmissionChannel};
use tokio::{try_join,task, sync::{MutexGuard, Mutex}, time::{Duration, timeout}}; 
use crate::{Actuator, Shared, Control};
use flume::*;
use std::{
    sync::{
        Arc, atomic::{
            AtomicI32, Ordering}}};

use manufacturer::{PIDSetup,actuator_data::PID, Actions, sensing_data::{Target, Actual, Readings}, Initiation};


impl Shared for PID{
    // defining the type of the lock for the implementation of this struct
    type Type= Arc<Mutex<(Actual,Vec<(Target,String, i32)>)>>;
    type SharedLock<'a>=MutexGuard<'a, (Actual,Vec<(Target,String, i32)>)>;
}
#[allow(non_snake_case)]
impl<'a> Actuator<'a> for PID{

    async fn actuator_control(sensing_info: Self::Type,sensor_recv: Receiver<ReadingType>,
        counts: Arc<AtomicI32>, feedback_send: Sender<ReadingType>, robotic_data: Readings) {
        if !sensor_recv.is_empty(){
            let receiver_lock=Arc::clone(&sensing_info);
            let sensor_recv_cloned=sensor_recv.clone();
            let feedback_send_cloned=feedback_send.clone();
            let robotic_data_cloned=robotic_data.clone();
            task::spawn(async move{
                match TransmissionChannel::recieve_transmission(sensor_recv_cloned).await{
                    Some(val)=>{
                        let lock=receiver_lock.lock().await;
                        let ReadingType::RoboticArm(arm, object, id)=val;
                        println!("object Id: {:?} Received", id);
                        Self::process_singals(lock, arm, object, id, feedback_send_cloned, robotic_data_cloned, counts).await;
                    },
                    None => (),
                } 
            }).await.unwrap();
        }
    }

    async fn process_singals(lock: Self::SharedLock<'a>, mut current_arm_status: Actual, mut object_status: Target,
        id: i32, feedback_send: Sender<ReadingType>, robotic_data: Readings, counts: Arc<AtomicI32>){
//TODO: processing Position
        let position=timeout(Duration::from_millis(1),task::spawn(async move{
            PID::calculate_pid(&mut current_arm_status.position,&mut object_status.position, "Position")
        }));
//TODO: processing Temparture
        let temparture=timeout(Duration::from_millis(1),task::spawn(async move{
            PID::calculate_pid(&mut current_arm_status.temperature,&mut object_status.temperature, "Temprature")
        }));
// TODO: processing Force
        let force=timeout(Duration::from_millis(1),task::spawn(async move{
            PID::calculate_pid(&mut current_arm_status.force,&mut object_status.force,  "Force")
        }));
//TODO: processing Velocity
        let velocity=timeout(Duration::from_millis(1),task::spawn(async move{
            PID::calculate_pid(&mut current_arm_status.velocity,&mut object_status.velocity,  "Velocity")
        }));
        //use tokio try join to check if the deadline is satisfied
        match try_join!(position, temparture, force, velocity){
            Ok(val)=>{
                println!("deadline is satisfied");
                Self::process_feedback(val.0.unwrap(), val.1.unwrap(), val.2.unwrap(), val.3.unwrap(), id, robotic_data,feedback_send, counts).await; 
            },
            Err(_err)=>{
                println!("time constraint was brocken.. entering safe mode");
                return;
            }
        }
        drop(lock);
    }

    async fn process_feedback(pos: f32, temparture: f32, force: f32, vel: f32, id_deleted: i32,
        mut robotic_data: Readings, feedback_send: Sender<ReadingType>, counts: Arc<AtomicI32>) {
       let updated_arm_status=Initiation::init(temparture, force, vel, pos);
       println!("Object with ID: {:?} is lifted", id_deleted);
       println!("Updated Arm stats: {:?}", updated_arm_status);
       let updated_readings= robotic_data.update_indices(id_deleted, updated_arm_status);
       let sensing_info= Arc::new(Mutex::new((updated_readings.current_state, updated_readings.objects.clone())));
       counts.fetch_sub(1, Ordering::Release);
       let value= counts.load(Ordering::Acquire);
       if value ==0{
            return;
       }
       println!("remaining objects: {:?}", counts);
       robotic_data.collect_data(sensing_info, feedback_send, counts).await;
    }
}

