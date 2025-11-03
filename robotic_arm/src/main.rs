use advanced_pid::{prelude::*, PidGain, Pid};
// use pid::Pid;
fn main() {
    let gain = PidGain {
        kp: 1.0,
        ki: 0.8,
        kd: 0.1,
    };
    let mut pid = Pid::new(gain.into());

    let target = 43.42;
    let mut actual = 2.0;
    let dt = 0.1;
    loop {
        // Calculate control output
        let output = pid.update(target, actual, dt);

        // Simulate the system response
        actual += (output - actual) / 4.0;
        println!("{:5.2}", actual);
        // Sleep 100ms
    
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    // println!("PID setpoint is : {:?}", pid.setpoint);
    // println!("PID limit is : {:?}", pid.p_limit);
    // // Test simple proportional
    // let value=pid.next_control_output(0.0);
    // let output= value.output - pid.setpoint;
    // println!("{:?}, output after deducting error: {:?}", value, output);
    // // Test proportional limit
    // let value_1=pid.next_control_output(10.0);
    // let output= value_1.output - pid.setpoint;
    // println!("{:?}, output after deducting error: {:?}", value_1, output);
    // let value_1=pid.next_control_output(30.0);
    // let output= value_1.output - pid.setpoint;
    // println!("{:?}, output after deducting error: {:?}", value_1, output);
}
