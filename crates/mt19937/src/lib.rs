#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use rand_core::{impls, RngCore, SeedableRng};

const LOWER_MASK: u32 = 0x7FFFFFFF;
const UPPER_MASK: u32 = !LOWER_MASK;

const ARRAY_SEED: u32 = 19650218;

const CONST_F: u32 = 1812433253;
const CONST_A: u32 = 0x9908B0DF;
const CONST_M: u32 = 397;
const CONST_N: usize = 624;

const CONST_F2: u32 = 1664525;
const CONST_F3: u32 = 1566083941;

const CONST_U: u32 = 11;
const CONST_D: u32 = 0xFFFFFFFF;

const CONST_S: u32 = 7;
const CONST_B: u32 = 0x9D2C5680;

const CONST_T: u32 = 15;
const CONST_C: u32 = 0xEFC60000;

const CONST_L: u32 = 18;

pub struct Mt19937Rng {
    pub state: [u32; CONST_N],
    pub index: usize,
}

impl SeedableRng for Mt19937Rng {
    type Seed = [u8; 16];

    fn from_seed(seed: Self::Seed) -> Self {
        Self::from_seed_array(seed)
    }

    fn seed_from_u64(seed: u64) -> Self {
        let mut state = [0u32; CONST_N];
        let index = CONST_N;

        state[0] = seed as u32;

        for i in 1..state.len() {
            state[i] = CONST_F
                .wrapping_mul(state[i - 1] ^ (state[i - 1] >> 30))
                .wrapping_add(i as u32);
        }

        Self { state, index }
    }
}

impl Mt19937Rng {
    fn from_seed_array<const N: usize>(seed: [u8; N]) -> Self
    where
        [(); N / 4]:,
    {
        let mut rng = Self::seed_from_u64(ARRAY_SEED as u64);

        // Convert array
        let mut dest = [0u32; N / 4];

        for (source, dest) in seed.chunks_exact(4).zip(dest.iter_mut()) {
            *dest = u32::from_be_bytes(<[u8; 4]>::try_from(source).unwrap());
        }
        let seed = dest;

        // Terrible C-like code because I used the reference code as is for this part.
        let mut i = 1;
        let mut j = 0;

        let start = if rng.state.len() > seed.len() {
            rng.state.len()
        } else {
            seed.len()
        };

        for _ in (0..start).rev() {
            rng.state[i] = (rng.state[i]
                ^ CONST_F2.wrapping_mul(rng.state[i - 1] ^ (rng.state[i - 1] >> 30)))
            .wrapping_add(seed[j])
            .wrapping_add(j as u32);

            i += 1;
            j += 1;

            if i >= rng.state.len() {
                rng.state[0] = rng.state[rng.state.len() - 1];
                i = 1;
            };
            if j >= seed.len() {
                j = 0;
            };
        }

        for _ in (1..rng.state.len()).rev() {
            rng.state[i] = (rng.state[i]
                ^ CONST_F3.wrapping_mul(rng.state[i - 1] ^ (rng.state[i - 1] >> 30)))
            .wrapping_sub(i as u32);

            i += 1;
            if i >= rng.state.len() {
                rng.state[0] = rng.state[rng.state.len() - 1];
                i = 1;
            };
        }

        rng.state[0] = 0x80000000;
        rng
    }

    fn twist(&mut self) {
        for i in 0..self.state.len() {
            let x = (self.state[i] & UPPER_MASK)
                | (self.state[(i + 1) % self.state.len()] & LOWER_MASK);

            let mut xa = x >> 1;

            if x & 1 == 1 {
                xa ^= CONST_A
            }

            self.state[i] =
                self.state[((i as u32 + CONST_M) % self.state.len() as u32) as usize] ^ xa;
        }

        self.index = 0;
    }
}

impl RngCore for Mt19937Rng {
    fn next_u32(&mut self) -> u32 {
        if self.index >= self.state.len() {
            self.twist();
        };

        let mut y = self.state[self.index];
        y ^= (y >> CONST_U) & CONST_D;
        y ^= (y << CONST_S) & CONST_B;
        y ^= (y << CONST_T) & CONST_C;
        y ^= y >> CONST_L;

        self.index += 1;
        y
    }

    fn next_u64(&mut self) -> u64 {
        impls::next_u64_via_u32(self)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        Ok(self.fill_bytes(dest))
    }
}
