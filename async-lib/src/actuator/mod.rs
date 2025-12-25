use crate::{Initiation, PIDSetup, Sensing, sensor::{Actual, ReadingType, Readings, Target}, transmission_control::TransmissionChannel};
use crate::{Actions, Actuator, Shared, Control};
use float_eq::float_eq;
use crossbeam::channel::*;
use std::{
    thread, time::Duration, sync::{
        Arc, Mutex, MutexGuard, atomic::{
            AtomicI32, Ordering}}};


impl Shared for Pid{
    // defining the type of the lock for the implementation of this struct
    type Type= Arc<Mutex<(Actual,Vec<(Target,String, i32)>)>>;
    type SharedLock<'a>=MutexGuard<'a, (Actual,Vec<(Target,String, i32)>)>;
}
impl<'a> Actuator<'a> for Pid{
    fn actuator_control(sensing_info: Self::Type,sensor_recv: Receiver<ReadingType>,
        counts: Arc<AtomicI32>, feedback_send: Sender<ReadingType>, robotic_data: Readings) {
            if sensor_recv.is_empty(){
                println!("No further updates required.. Robotic Arm is updated");
                thread::sleep(Duration::from_millis(200));
            }
            else {
                let value=counts.load(Ordering::Acquire);
                for _ in 0..value{
                    let receiver_lock=Arc::clone(&sensing_info);
                    let sensor_recv_cloned=sensor_recv.clone();
                    let feedback_send_cloned=feedback_send.clone();
                    let robotic_data_cloned=robotic_data.clone();
                    thread::spawn(move || {
                        match TransmissionChannel::recieve_transmission(sensor_recv_cloned){
                            Some(val)=>{
                                let lock=receiver_lock.lock().unwrap();
                                let ReadingType::RoboticArm(arm, object, id)=val;
                                println!("object Id: {:?} Received", id);
                                Self::process_singals(lock, arm, object, id, feedback_send_cloned, robotic_data_cloned, counts);
                            },
                            None => (),
                        } 
                    });
                    return;
                }
            }
    }

    fn process_singals(lock: Self::SharedLock<'a>, mut current_arm_status: Actual, mut object_status: Target,
        id: i32, feedback_send: Sender<ReadingType>, robotic_data: Readings, counts: Arc<AtomicI32>){
//TODO: processing Position
        let position=thread::spawn(move||{
            PidGain::calculate_pid(&mut current_arm_status.position,&mut object_status.position,1, "Position")
        }).join().unwrap();
//TODO: processing Temparture
        let temparture=thread::spawn(move || {
            PidGain::calculate_pid(&mut current_arm_status.temperature,&mut object_status.temperature, 1, "Temprature")
        }).join().unwrap();
// //TODO: processing Force
        let force=thread::spawn(move || {
            PidGain::calculate_pid(&mut current_arm_status.force,&mut object_status.force, 1, "Force")
        }).join().unwrap();
//TODO: processing Velocity
        let velocity=thread::spawn(move ||{
            PidGain::calculate_pid(&mut current_arm_status.velocity,&mut object_status.velocity, 1, "Velocity")
        }).join().unwrap();

        Self::process_feedback(position, temparture, force, velocity, id, robotic_data,feedback_send, counts); 
        // recv_counts.fetch_add(1, Ordering::Release);

        drop(lock);
    }

    fn process_feedback(pos: f32, temparture: f32, force: f32, vel: f32, id_deleted: i32,
        mut robotic_data: Readings, feedback_send: Sender<ReadingType>, counts: Arc<AtomicI32>) {
       let updated_arm_status=Initiation::init(temparture, force, vel, pos);
       println!("Object with ID: {:?} is lifted", id_deleted);
       println!("Updated Arm stats: {:?}", updated_arm_status);
       let updated_readings= robotic_data.update_indices(id_deleted, updated_arm_status);
       let sensing_info= Arc::new(Mutex::new((updated_readings.current_state, updated_readings.objects.clone())));
       counts.fetch_sub(1, Ordering::Release);
       robotic_data.collect_data(sensing_info, feedback_send, counts);
    }
}

