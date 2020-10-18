use rand::{
    distributions::{uniform::SampleUniform, Distribution, Standard},
    Rng,
};
use rand_pcg::Pcg64Mcg;
use rand_seeder::Seeder;

pub struct SeededRng {
    pub seed: String,
    rng: Pcg64Mcg,
}

impl SeededRng {
    pub fn new(seed: &str) -> Self {
        let rng: Pcg64Mcg = Seeder::from(&seed).make_rng();
        Self {
            rng,
            seed: seed.to_string(),
        }
    }

    pub fn gen<T>(&mut self) -> T
    where
        Standard: Distribution<T>,
    {
        self.rng.gen::<T>()
    }

    /// Returns a random value within the specific range, where max is excluded, and min is included.
    pub fn gen_range<T>(&mut self, min: T, max: T) -> T
    where
        T: SampleUniform,
    {
        self.rng.gen_range(min, max)
    }

    pub fn random_entry<'a, T>(&mut self, entries: &'a [T]) -> Option<&'a T> {
        if entries.is_empty() {
            return None;
        }

        let count = entries.len();
        if count == 1 {
            Some(&entries[0])
        } else {
            Some(&entries[self.gen_range(0, count)])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_returns_type_t() {
        let mut rng = SeededRng::new("test");
        rng.gen::<i32>();
        // Assert no panic
        assert!(true);
    }

    #[test]
    fn gen_range_does_not_return_out_of_range() {
        let mut rng = SeededRng::new("test");

        for _ in 0..100 {
            let gen = rng.gen_range(0, 5);
            assert!(gen >= 0 && gen < 5);
        }
    }

    #[test]
    fn random_entry_when_empty() {
        let mut rng = SeededRng::new("test");
        let entries: Vec<i32> = vec![];

        assert!(rng.random_entry(&entries).is_none());
    }

    #[test]
    fn random_entry_when_not_empty() {
        let mut rng = SeededRng::new("test");
        let entries: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        for _ in 0..100 {
            match rng.random_entry(&entries) {
                None => assert!(false),
                Some(entry) => assert!(entries.contains(entry)),
            }
        }
    }
}
