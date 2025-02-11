use rand::random;
use std::time::Instant;

struct Sensor {
    id: u32,
    value: f32,
}

fn main() {
    let n = 1_000_000;

    // Tidtagning for at oprette vektoren
    let start = Instant::now();

    let mut sensors: Vec<Sensor> = Vec::with_capacity(n);
    for id in 0..n {
        let value: f32 = random::<f32>() * 100.0;
        sensors.push(Sensor { id: id as u32, value });
    }

    let creation_duration = start.elapsed();
    println!("Tid brugt til at oprette vektor med data (i nanosekunder): {}", creation_duration.as_nanos());

    // Tidtagning for at søge efter et specifikt sensor-ID
    let search_id = 3;
    //let search_id = 50000;

    let start = Instant::now();

    if let Some(sensor) = sensors.iter().find(|&sensor| sensor.id == search_id) {
        println!("Fundet sensor med id {} og værdi {}", sensor.id, sensor.value);
    } else {
        println!("Sensor med id {} ikke fundet.", search_id);
    }

    let search_duration = start.elapsed();
    println!("Tid brugt til at søge efter sensor i Rust (i nanosekunder): {}", search_duration.as_nanos());
}
