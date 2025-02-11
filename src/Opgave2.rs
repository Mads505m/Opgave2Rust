use std::collections::HashMap;
use rand::random;
use std::time::Instant;

struct Sensor {
    id: u32,
    value: f32,
}

fn main() {
    let n = 1_000_000;
    let mut sensors: HashMap<u32, Sensor> = HashMap::with_capacity(n);

    for id in 0..n {
        let value: f32 = random::<f32>() * 100.0;
        sensors.insert(id as u32, Sensor { id: id as u32, value });
    }

    let search_id = 3;

    let start = Instant::now();

    if let Some(sensor) = sensors.get(&search_id) {
        println!("Fundet sensor med id {} og værdi {}", sensor.id, sensor.value);
    } else {
        println!("Sensor med id {} ikke fundet.", search_id);
    }

    let duration = start.elapsed();

    println!("Tid brugt til at søge efter sensor i Rust (i nanosekunder): {}", duration.as_nanos());
}
