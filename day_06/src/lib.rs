use std::fs;

pub fn input_lantern_fish_school() -> LanternFishSchool {
    let fish = fs::read_to_string("day_06/input")
        .expect("Failed to read input file")
        .trim_end()
        .split(',')
        .map(|num_str| num_str.parse::<usize>().expect("Bad input"))
        .collect::<Vec<_>>();
    LanternFishSchool::new(&fish)
}

pub struct LanternFishSchool {
    /// Stores the number of fish of each time period before birth
    inner: [usize; LanternFishSchool::MAX_AGE + 1],
}

impl LanternFishSchool {
    const MAX_AGE: usize = 8;
    const BIRTH_RESET_TO: usize = 6;
    const NUMBER_OF_BABIES: usize = 1;

    fn new(fish: &[usize]) -> Self {
        let mut ages = [0; 9];
        fish.iter().for_each(|index| ages[*index] += 1);
        LanternFishSchool { inner: ages }
    }

    pub fn simulate_once(&mut self) {
        let new_borns = self.inner[0] * Self::NUMBER_OF_BABIES;
        self.inner[..=Self::BIRTH_RESET_TO].rotate_left(1);
        self.inner[Self::BIRTH_RESET_TO] += self.inner[7];
        self.inner[7] = self.inner[Self::MAX_AGE];
        self.inner[Self::MAX_AGE] = new_borns;
    }

    pub fn simulate_n(&mut self, days: usize) {
        for _ in 0..days {
            self.simulate_once();
        }
    }

    pub fn count(&self) -> usize {
        self.inner.iter().sum()
    }
}

#[cfg(test)]
mod test {
    use crate::LanternFishSchool;

    #[test]
    fn simulate() {
        let mut school = LanternFishSchool::new(&[3,4,3,1,2][..]);
        school.simulate_once();
        assert_eq!(
            school.inner,
            [1, 1, 2, 1, 0, 0, 0, 0, 0],
        );
        school.simulate_once();
        assert_eq!(
            school.inner,
            [1, 2, 1, 0, 0, 0, 1, 0, 1],
        );
    }
}
