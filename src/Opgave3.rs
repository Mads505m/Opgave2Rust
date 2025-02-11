use rand::random;
use std::time::Instant;

struct Sensor {
    id: u32,
    value: f32,
}

fn main() {
    let n = 1_000_000;
    let mut sensors: Vec<Box<Sensor>> = Vec::with_capacity(n);

    for id in 0..n {
        let value: f32 = random::<f32>() * 100.0;
        let sensor = Box::new(Sensor { id: id as u32, value });
        sensors.push(sensor);
    }

    let search_id = 3;

    let start = Instant::now();

    if let Some(sensor) = sensors.get(search_id as usize) {
        println!("Sensor ID: {}, Værdi: {}", sensor.id, sensor.value);
    } else {
        println!("Sensor med id {} ikke fundet.", search_id);
    }

    let duration = start.elapsed();

    println!("Tid brugt til at allokere data med Box i Rust (i nanosekunder): {}", duration.as_nanos());
}
