use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
}

fn simulated_expensive_calculation(intensisy: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensisy
}

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps",
            simulated_expensive_calculation(intensity)
        );
    } else {
        println!(
            "Today, run for {} minutes!",
            simulated_expensive_calculation(intensity)
        );
    }
}
