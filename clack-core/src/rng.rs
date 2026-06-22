use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, LogNormal, Uniform};

pub struct ClackRng {
    rng: ChaCha8Rng,
}

impl ClackRng {
    pub fn new(seed: Option<u64>) -> Self {
        let rng = match seed {
            Some(s) => ChaCha8Rng::seed_from_u64(s),
            None => ChaCha8Rng::from_entropy(),
        };
        Self { rng }
    }

    pub fn sample_log_normal(&mut self, mu: f64, sigma: f64) -> f64 {
        if sigma <= 0.0 {
            return mu.exp();
        }
        let dist = LogNormal::new(mu, sigma).unwrap();
        dist.sample(&mut self.rng)
    }

    pub fn sample_uniform(&mut self, min: f64, max: f64) -> f64 {
        let dist = Uniform::new_inclusive(min, max);
        dist.sample(&mut self.rng)
    }

    pub fn sample_uniform_int(&mut self, min: usize, max: usize) -> usize {
        let dist = Uniform::new_inclusive(min, max);
        dist.sample(&mut self.rng)
    }

    pub fn sample_bool(&mut self, probability: f64) -> bool {
        self.rng.gen_bool(probability.clamp(0.0, 1.0))
    }
}
