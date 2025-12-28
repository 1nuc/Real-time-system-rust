use lapin::*;
use tokio::{time::sleep, *};
use std::time::Duration;

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

#[tokio::main]
async fn main() {
    
    
    let connection: Connection= create_connection().await;
    //create a communication channel
    let channel=connection.create_channel().await.expect("error in creating a channel");

}
