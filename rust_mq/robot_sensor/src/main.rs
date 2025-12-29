use lapin::{options::*,types::FieldTable, *};
use tokio::{time::sleep, *, sync::Mutex};
use std::{sync::{Arc,atomic::{AtomicI32, Ordering}},time::Duration};
use manufacturer::*;
use serde_json::{self};
mod sensor;

async fn create_connection()-> Connection{
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

#[allow(unused_variables)]
#[allow(non_snake_case)]
#[tokio::main]
async fn main() {
    
    
    let connection: Connection= create_connection().await;
    //create a communication channel
    let channel=connection.create_channel().await.expect("error in creating a channel");
    let _=channel.confirm_select(ConfirmSelectOptions::default()).await;
    let _=channel.queue_declare("sensing_data",QueueDeclareOptions::default(), FieldTable::default()).await;
    let objects=sensing_data::Readings::assign_data(50).filter_noise();
    let packets=Arc::new(Mutex::new((objects.current_state, objects.objects.clone())));
    let counter=Arc::new(AtomicI32::new(objects.objects_num));
    let counter_cloned=Arc::clone(&counter);
    let value= counter_cloned.load(Ordering::Acquire);
    for i in 0..value{
        let channel_clone=channel.clone();
        let counter_cloned=Arc::clone(&counter);
        let packets_cloned=Arc::clone(&packets);
        task::spawn(async move{
            let mut data=packets_cloned.lock().await;
            match data.1.pop(){
                Some(val)=>{
                    let data_sered=serde_json::to_vec(&sensor::ReadingType::RoboticArm(data.0, val.0, val.2)
                        ).expect("unable to serialize the data");
                    println!("sending robotic data");
                    let confirmation=channel_clone.basic_publish("", "sensing_data", BasicPublishOptions::default(), &data_sered,BasicProperties::default()).await.expect("error");
                    let confirmed=confirmation.await.expect("error");
                    match confirmed{
                        publisher_confirm::Confirmation::Ack(msg)=>{
                            println!("Message has been confirmed");
                        },
                        publisher_confirm::Confirmation::Nack(msg)=>{
                            println!("Message has not yet been confirmed");
                        },
                        publisher_confirm::Confirmation::NotRequested=>{
                            println!("Message is waiting to be requested");
                        }
                    }
                    counter_cloned.fetch_sub(1,Ordering::Release);
                },
                None =>{
                    println!("All boxes have been sent");
                    drop(data);
                }
            }
        });
    }
    signal::ctrl_c().await.expect("failed");
}
