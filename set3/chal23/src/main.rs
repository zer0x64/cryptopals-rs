use mt19937::Mt19937Rng;
use rand::prelude::*;

const CONST_U: u32 = 11;

const CONST_S: u32 = 7;
const CONST_B: u32 = 0x9D2C5680;

const CONST_T: u32 = 15;
const CONST_C: u32 = 0xEFC60000;

const CONST_L: u32 = 18;

fn main() {
    // Get rng output
    let mut rng = Mt19937Rng::from_entropy();
    let mut output = [0u32; 624];

    rng.fill(output.as_mut_slice());

    for y in output.iter_mut() {
        *y ^= *y >> CONST_L;

        *y ^= (*y << CONST_T) & CONST_C;

        let mut intermediate = 0u32;
        for i in 0..(32 / CONST_S + 1) {
            let mask = ((1 << (CONST_S)) - 1) << (CONST_S * i);
            intermediate |= (*y ^ ((intermediate << CONST_S) & CONST_B)) & mask;
        }

        *y = intermediate;

        // let mut intermediate = *y;
        // for _ in 0..3 {
        //     intermediate = *y ^ (intermediate >> CONST_U);
        // }

        let mut intermediate = 0u32;
        for i in 0..(32 / CONST_U + 1) {
            let mask = ((1 << (CONST_U)) - 1) << (CONST_U * (32 / CONST_U - i));
            intermediate |= (*y ^ (intermediate >> CONST_U)) & mask;
        };

        *y = intermediate;
    }

    let mut rng2 = Mt19937Rng::from_entropy();
    rng2.state = output;

    let mut output1 = [0u8; 1000];
    let mut output2 = [0u8; 1000];

    rng.fill(output1.as_mut_slice());
    rng2.fill(output2.as_mut_slice());

    assert_eq!(output1, output2);

    println!("It works!")
}
