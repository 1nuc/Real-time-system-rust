use tokio::{sync::Mutex, *, time::sleep};
use manufacturer::{Initiation, PIDSetup, actuator_data::PID, sensing_data::{Actual,Target}};
use serde::{Serialize, Deserialize};
use std::sync::{Arc, atomic::{AtomicI32, Ordering}};
use lapin::{BasicProperties, Channel, Connection, ConnectionProperties, options::*, publisher_confirm::Confirmation, types::FieldTable};
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
pub enum ReadingType{
    RoboticArm(Actual, Target, i32),
} 
type SensingType=Vec<(Actual, Target, i32)>;

pub async fn create_connection()-> Connection{
    let addr="amqp://guest:guest@localhost:5672";
    let mut res=Connection::connect(addr,ConnectionProperties::default()).await;
    while res.is_err(){
        println!("Failure in Connecting");
        res=Connection::connect(addr,ConnectionProperties::default()).await;
        sleep(Duration::from_secs(1)).await;
    }
    println!("Successful connection");
    let connection= res.unwrap();
    connection
    
}
pub async fn receive(data_vec: Vec<(Actual, Target, i32)>, connection: Connection){
    let (arm, object, id)=find_smallest(data_vec.clone());
    let handle=task::spawn(async move {
        println!("Processing the Nearset Object with ID:{:?}", id );
        process_singals(data_vec, arm, object, id,connection).await;
    });
    handle.await.unwrap();
}

fn find_smallest(vector: Vec<(Actual, Target, i32)>)-> (Actual, Target, i32){
    let extracted_vals: Vec<_>=vector.clone().into_iter().map(|x|{
        let y=x.0.position.abs() -x.1.position.abs();
        (y, x.2)
    }).collect();
    let smallest_val=extracted_vals.into_iter().min_by(|a,b|a.0.partial_cmp(&b.0).unwrap()).unwrap();
    let mut filtered_vec: Vec<(Actual, Target, i32)>=vector.into_iter().filter(|x| x.2 ==smallest_val.1).collect();
    filtered_vec.pop().unwrap()
}

// //TODO: Add another function to update the indices without the need for the object data to be
// //available
pub async fn process_singals(robotic_data: SensingType,mut current_arm_status: Actual, mut object_status: Target,
    id: i32, connection: Connection){
//TODO: processing Position
    let position=task::spawn(async move{
        PID::calculate_pid(&mut current_arm_status.position,&mut object_status.position, "Position")
    }).await.unwrap();
//TODO: processing Temparture
    let temparture=task::spawn(async move{
        PID::calculate_pid(&mut current_arm_status.temperature,&mut object_status.temperature, "Temprature")
    }).await.unwrap();
// //TODO: processing Force
    let force=task::spawn(async move{
        PID::calculate_pid(&mut current_arm_status.force,&mut object_status.force,  "Force")
    }).await.unwrap();
//TODO: processing Velocity
    let velocity=task::spawn(async move{
        PID::calculate_pid(&mut current_arm_status.velocity,&mut object_status.velocity,  "Velocity")
    }).await.unwrap();

    process_feedback(position, temparture, force, velocity, id, robotic_data, connection).await; 
}

fn update_values(object_id: i32, arm_status: Actual, objects: SensingType) -> SensingType{
    let mut updated_data: SensingType=objects.clone().into_iter().filter(|x| x.2 !=object_id).collect();
    let mut updated_vector=vec![];
    while let Some(x) =updated_data.pop(){
        updated_vector.push((arm_status, x.1,x.2));
    }
   updated_vector 
}

async fn process_feedback(pos: f32, temparture: f32, force: f32, vel: f32, id_deleted: i32,
    robotic_data: SensingType, connection: Connection) {
   let updated_arm_status=Actual::init(temparture, force, vel, pos);
   println!("Object with ID: {:?} is lifted", id_deleted);
   println!("Updated Arm stats: {:?}", updated_arm_status);
   let update_readings=update_values(id_deleted, updated_arm_status, robotic_data);
   send_feedback(update_readings, connection).await;
}

pub async fn create_channel(connection: Connection)-> Channel{
    let channel=connection.create_channel().await.expect("error in creating a channel");
    let _=channel.confirm_select(ConfirmSelectOptions::default()).await;
    let _=channel.queue_declare("feedback_data",QueueDeclareOptions::default(), FieldTable::default()).await;
    channel
}

async fn handle_transmission(channel: Channel,counter: Arc<AtomicI32>, data: (Actual, Target, i32)){
    let data_sered=serde_json::to_vec(&(data)
        ).expect("unable to serialize the data");
    println!("sending robotic data");
    let confirmation=channel.basic_publish(
        "", "feedback_data",
        BasicPublishOptions::default(),
        &data_sered,BasicProperties::default()).await.expect("error");
    let confirmed=confirmation.await.expect("error");
    get_confirmation(confirmed).await;
    counter.fetch_sub(1,Ordering::Release);
}

async fn get_confirmation(confirmed: Confirmation)-> String{

    match confirmed{
        Confirmation::Ack(_msg)=>{
            "Feeback message have been sent and approved".to_string()
        },
        Confirmation::Nack(_msg)=>{
            "Message has not yet been confirmed".to_string()
        },
        Confirmation::NotRequested=>{
            "Message is waiting to be requested".to_string()
        }
    }
}

#[allow(non_snake_case)]
pub async fn send_feedback(data: SensingType, connection: Connection){
    let channel=create_channel(connection).await;
    let packets=Arc::new(Mutex::new(data.clone()));
    let counter=Arc::new(AtomicI32::new(data.len().try_into().unwrap()));
    let counter_cloned=Arc::clone(&counter);
    let value= counter_cloned.load(Ordering::Acquire);
    println!("Sending Feedback:{:?} objects", value);
    for _ in 0..value{
        let channel_cloned=channel.clone();
        let counter_cloned=Arc::clone(&counter);
        let packets_cloned=Arc::clone(&packets);
        task::spawn(async move{
            let mut data=packets_cloned.lock().await;
            match data.pop(){
                Some(val)=>{
                    handle_transmission(channel_cloned, counter_cloned, val).await;
                },
                None =>{
                    println!("All boxes have been sent");
                    drop(data);
                }
            }
        });
    }
}
