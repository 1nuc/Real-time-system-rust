use lapin::{options::*, publisher_confirm::Confirmation, types::FieldTable, *};
use futures_lite::stream::StreamExt;
use tokio::{time::{sleep, timeout}, *, sync::Mutex};
use std::{sync::{Arc,atomic::{AtomicI32, Ordering}},time::Duration};
use manufacturer::{sensing_data::{Actual, Readings, Target}, *};
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
pub async fn create_channel(connection: Arc<Connection>)-> Channel{
    let channel=connection.create_channel().await.expect("error in creating a channel");
    let _=channel.confirm_select(ConfirmSelectOptions::default()).await;
    // let _=channel.queue_delete("feedback_data", QueueDeleteOptions::default()).await.expect("unable to delete the queue");
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

#[allow(non_snake_case)]
async fn handle_feedback(consumer: Result<Consumer>)-> Vec<(Actual, Target,i32)>{
    let mut data_vec=vec![];
    loop{
        match timeout(Duration::from_millis(200), consumer.clone().expect("Error retreiving the data").next()).await{
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
                break;
            },
            Err(_)=>{
                println!("waiting for a feedback from the sensor");
                break;
            },
        }
    }
    data_vec
}
#[allow(non_snake_case)]
pub async fn sensing(channel: Channel, data: Arc<Mutex<(Actual, Vec<(Target, String,i32)>)>>, counter: Arc<AtomicI32>, connection: Arc<Connection>){
    let counter_cloned=Arc::clone(&counter);
    let value= counter_cloned.load(Ordering::Acquire);
    println!("Sending:{:?} objects", value);
    let mut handler=vec![];
    for _ in 0..value{
        let channel_cloned=channel.clone();
        let counter_cloned=Arc::clone(&counter);
        let packets_cloned=Arc::clone(&data);
        let handle=task::spawn(async move{
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
        handler.push(handle);
    }
    for handle in handler{
        handle.await.unwrap();
    }
    if value==1{
        let _=connection.close(200,"terminating gracefully").await;
    }
}

pub async fn sensor_control(channel: Channel, connection: Arc<Connection>){
    let objects=sensing_data::Readings::assign_data(50).filter_noise();
    let packets=Arc::new(Mutex::new((objects.current_state, objects.objects.clone())));
    let counter=Arc::new(AtomicI32::new(objects.objects_num));
    sensing(channel, packets, counter, Arc::clone(&connection)).await;
    let channel=create_channel(Arc::clone(&connection)).await;
    loop{
        sleep(Duration::from_secs(1)).await;
        if connection.status().state()!=ConnectionState::Connected{
            println!("All objects have been sent..server is closing");
            return;
        }
        let mut consumer= channel.basic_consume("feedback_data", "sensor", BasicConsumeOptions::default(), FieldTable::default()).await;
        while consumer.is_err() {
             println!("Waiting for a message to recieve");
             consumer= channel.basic_consume("feedback_data", "sensor", BasicConsumeOptions::default(), FieldTable::default()).await;
             sleep(Duration::from_secs(1)).await;
        }
        let data=handle_feedback(consumer).await;
        if data.is_empty(){
            continue;
        }
        let mut data_vec=vec![];
        for i in data.clone().into_iter(){
            data_vec.push((i.1, Readings::TOKEN.to_string(), i.2));
        }
        let packets=Arc::new(Mutex::new((data[0].0, data_vec.clone())));
        let counter=Arc::new(AtomicI32::new(data.len().try_into().unwrap()));
        sensing(channel.clone(), packets, counter, connection.clone()).await;
    }
}
