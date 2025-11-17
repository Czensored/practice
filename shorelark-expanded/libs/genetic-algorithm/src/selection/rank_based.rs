use crate::*;

#[derive(Clone, Debug, Default)]
pub struct RankBasedSelection;


impl SelectionMethod for RankBasedSelection {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual,
    {
        assert!(!population.is_empty(), "got an empty population");

        let mut ranked: Vec<&I> = population.iter().collect();

        ranked.sort_by(|a, b| a.fitness().partial_cmp(&b.fitness()).unwrap());

        let base: f32 = 2.0;

        ranked
            .choose_weighted(rng, |individual| {
                let rank = ranked.iter().position(|&x| x.fitness() == individual.fitness()).unwrap() + 1;
                base.powi(rank as i32)
            })
            .expect("population should not be empty")
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use std::collections::BTreeMap;

    #[test]
    fn rank_based_selection() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let population = vec![
            TestIndividual::new(2.0),
            TestIndividual::new(1.0),
            TestIndividual::new(4.0),
            TestIndividual::new(3.0),
        ];
        let mut actual_histogram = BTreeMap::new();

        //          /--| nothing special about this thousand;
        //          v  | a number as low as fifty might do the trick, too
        for _ in 0..1000 {
            let fitness = RankBasedSelection
                .select(&mut rng, &population)
                .fitness() as i32;

            *actual_histogram.entry(fitness).or_insert(0) += 1;
        }

        let expected_histogram = BTreeMap::from_iter([
            // (fitness, how many times this fitness has been chosen)
            (1, 72),
            (2, 130),
            (3, 273),
            (4, 525),
        ]);

        assert_eq!(actual_histogram, expected_histogram);
    }
}
