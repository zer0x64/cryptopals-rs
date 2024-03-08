use std::{
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use rand::prelude::*;

fn black_box() -> u32 {
    random_wait();

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backward")
        .as_secs();

    println!("Seed from blackbox: {}", timestamp);
    let mut rng = mt19937::Mt19937Rng::seed_from_u64(timestamp);

    random_wait();

    rng.next_u32()
}

fn random_wait() {
    let wait_sec: u64 = thread_rng().gen_range(40..1000);

    let wait_duration = Duration::from_secs(wait_sec);

    thread::sleep(wait_duration);
}

fn main() {
    let output = black_box();

    for i in 0..2000 {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backward");
        let timestamp = timestamp - Duration::from_secs(i);

        let mut rng = mt19937::Mt19937Rng::seed_from_u64(timestamp.as_secs());

        if rng.next_u32() == output {
            println!("Cracked seed: {}", timestamp.as_secs());
            break;
        }
    }
}
