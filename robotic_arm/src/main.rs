use advanced_pid::{prelude::*,i_pd::Ipd, PidGain, PidConfig};
use pid::Pid;
fn main() {
   // let gain =PidConfig::new(0.5, 0.2, 0.1).with_limits(-1.0, 1.0);
   // // let gain =PidGain{ kp: 0.1, ki: 0.3, kd: 0.1};
   // let mut pid =Pid::new(15.0, 100.0);
   // // let mut pid =Ipd::new(gain.into());
   // pid.p(10.0, 100.0);
   // let target=0.1;
   // let mut actual=0.0;
   // let dt=0.1;
   //
   //
   // for i in 0..50 {
   //      let mut output=pid.update(target, actual, dt);
   //      actual = (output - actual);
   //      println!("the output is : {:?}, and the result of the pid is: {:?}", actual, output);
   //  }
    let mut pid = Pid::new(5.0, 100.0);
    pid.p(0.0, 100.0).i(2.0, 100.0).d(0.0, 100.0);

    println!("PID setpoint is : {:?}", pid.setpoint);
    println!("PID limit is : {:?}", pid.p_limit);
    // Test simple proportional
    let value=pid.next_control_output(0.0);
    let output= value.output - pid.setpoint;
    println!("{:?}, output after deducting error: {:?}", value, output);
    // Test proportional limit
    let value_1=pid.next_control_output(10.0);
    let output= value_1.output - pid.setpoint;
    println!("{:?}, output after deducting error: {:?}", value_1, output);
    let value_1=pid.next_control_output(30.0);
    let output= value_1.output - pid.setpoint;
    println!("{:?}, output after deducting error: {:?}", value_1, output);
}
