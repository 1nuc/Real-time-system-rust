use tokio::*;
use manufacturer::*;

async fn prepare_data(){
    //assign 50 boxes of data
    let robotic_data=manufacturer::sensing_data::Readings::assign_data(50);
}
