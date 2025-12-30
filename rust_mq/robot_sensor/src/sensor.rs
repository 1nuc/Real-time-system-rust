use lapin::{options::*, publisher_confirm::Confirmation, types::FieldTable, *};
use futures_lite::stream::StreamExt;
use tokio::{time::{sleep, timeout}, *, sync::Mutex};
use std::{sync::{Arc,atomic::{AtomicI32, Ordering}},time::Duration};
use manufacturer::{sensing_data::{Actual, Target}, *};
use serde_json::{self};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum ReadingType{
    RoboticArm(Actual,Target, i32),
} 

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

async fn get_confirmation(confirmed: Confirmation)-> String{

    match confirmed{
        publisher_confirm::Confirmation::Ack(_msg)=>{
            "Message has been confirmed".to_string()
        },
        publisher_confirm::Confirmation::Nack(_msg)=>{
            "Message has not yet been confirmed".to_string()
        },
        publisher_confirm::Confirmation::NotRequested=>{
            "Message is waiting to be requested".to_string()
        }
    }
}
pub async fn create_channel(connection: Connection)-> Channel{
    let channel=connection.create_channel().await.expect("error in creating a channel");
    let _=channel.confirm_select(ConfirmSelectOptions::default()).await;
    let _=channel.queue_declare("sensing_data",QueueDeclareOptions::default(), FieldTable::default()).await;
    channel
}

async fn handle_transmission(channel: Channel,counter: Arc<AtomicI32>, arm_data: Actual, objects: Target, id: i32){
    let data_sered=serde_json::to_vec(&ReadingType::RoboticArm(arm_data, objects, id)
        ).expect("unable to serialize the data");
    println!("sending robotic data");
    let confirmation=channel.basic_publish(
        "", "sensing_data",
        BasicPublishOptions::default(),
        &data_sered,BasicProperties::default()).await.expect("error");
    let confirmed=confirmation.await.expect("error");
    get_confirmation(confirmed).await;
    counter.fetch_sub(1,Ordering::Release);
}

async fn handle_feedback(consumer: Result<Consumer>){
    let mut data_vec=vec![];
    loop{
        match timeout(Duration::from_secs(1), consumer.clone().expect("Error retreiving the data").next()).await{
            Ok(Some(msg))=>{
                if let Ok(msg)=msg{
                    let ReadingType::RoboticArm(arm,object,id)=serde_json::from_slice::<ReadingType>(&(msg.data)).expect("Unable to serialize the data");
                    data_vec.push((arm, object, id));
                    println!("Message recieved, Arm current position:{:?}, Objcet with ID:{:?}, stats:{:?}",arm, id, object);
                    let _=msg.acker.ack(BasicAckOptions::default()).await;
                }
            },
            Ok(None) =>{
                println!("messages have been received");
                receive(data_vec.clone(), Arc::clone(&connection)).await;
            },
            Err(_)=>{
                println!("Timeout");
                break;
            },
        }
    }
}
pub async fn sensor_control(channel: Channel){
    let mut consumer= channel.basic_consume("feedback_data", "Actuator", BasicConsumeOptions::default(), FieldTable::default()).await;
    if consumer.is_err(){
        let objects=sensing_data::Readings::assign_data(50).filter_noise();
        let packets=Arc::new(Mutex::new((objects.current_state, objects.objects.clone())));
        let counter=Arc::new(AtomicI32::new(objects.objects_num));
        let counter_cloned=Arc::clone(&counter);
        let value= counter_cloned.load(Ordering::Acquire);
        println!("Sending:{:?} objects", value);
        for _ in 0..value{
            let channel_cloned=channel.clone();
            let counter_cloned=Arc::clone(&counter);
            let packets_cloned=Arc::clone(&packets);
            task::spawn(async move{
                let mut data=packets_cloned.lock().await;
                match data.1.pop(){
                    Some(val)=>{
                        handle_transmission(channel_cloned, counter_cloned, data.0, val.0, val.2).await;
                    },
                    None =>{
                        println!("All boxes have been sent");
                        drop(data);
                    }
                }
            });
        }
    }
}
