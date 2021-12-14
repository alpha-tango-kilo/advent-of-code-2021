pub mod naive {
    use std::collections::HashMap;
    use std::fs;

    pub type Rules = HashMap<(char, char), [char; 3]>;

    pub fn input_polymer_rules() -> (Polymer, Rules) {
        let input = fs::read_to_string("day_14/input")
            .expect("Failed to read input file");
        string_polymer_rules(&input)
    }

    pub(crate) fn string_polymer_rules(s: &str) -> (Polymer, Rules) {
        let mut lines = s.lines();

        let polymer = Polymer {
            inner: lines.next().expect("Bad input").chars().collect(),
        };

        let rules = lines
            .skip(1) // blank line
            .map(|line| {
                let mut chars = line.chars();
                let a = chars.next().expect("Bad input");
                let b = chars.next().expect("Bad input");
                let mut chars = chars.skip(" -> ".len());
                let c = chars.next().expect("Bad input");
                ((a, b), [a, c, b])
            })
            .collect::<HashMap<_, _>>();

        (polymer, rules)
    }

    #[derive(Debug)]
    pub struct Polymer {
        inner: Vec<char>,
    }

    impl Polymer {
        pub fn pair_insertion(&mut self, rules: &Rules) {
            let last = *self.inner.last().unwrap();
            self.inner = self
                .inner
                .windows(2)
                .flat_map(|slice| {
                    &rules
                        .get(&(slice[0], slice[1]))
                        .expect("No rule for this expansion")[..2]
                })
                .cloned()
                .collect();
            self.inner.push(last);
        }

        pub fn pair_insertion_many(&mut self, rules: &Rules, times: usize) {
            for n in 1..=times {
                println!("Expansion {} / {}", n, times);
                self.pair_insertion(rules);
            }
        }

        pub fn freq_map(&self) -> HashMap<char, usize> {
            let mut map = HashMap::new();
            for c in &self.inner {
                *map.entry(*c).or_default() += 1;
            }
            map
        }
    }

    #[cfg(test)]
    impl ToString for Polymer {
        fn to_string(&self) -> String {
            self.inner.iter().collect()
        }
    }
}

pub mod smart_perhaps {
    use std::collections::HashMap;
    use std::fs;

    pub type Rules = HashMap<(char, char), ((char, char), (char, char))>;
    pub type FreqMap = HashMap<char, usize>;

    pub fn input_chars_rules() -> (Vec<char>, Rules) {
        let input = fs::read_to_string("day_14/input")
            .expect("Failed to read input file");
        string_chars_rules(&input)
    }

    fn string_chars_rules(s: &str) -> (Vec<char>, Rules) {
        let mut lines = s.lines();
        let chars = lines.next().expect("Bad input").chars().collect();

        let rules = lines
            .skip(1) // blank line
            .map(|line| {
                let mut chars = line.chars();
                let a = chars.next().expect("Bad input");
                let b = chars.next().expect("Bad input");
                let mut chars = chars.skip(" -> ".len());
                let c = chars.next().expect("Bad input");
                ((a, b), ((a, c), (c, b)))
            })
            .collect();
        (chars, rules)
    }

    pub fn freq_map(start: &[char], rules: &Rules, depth: usize) -> FreqMap {
        let mut freq_map = HashMap::new();
        start.windows(2).enumerate().for_each(|(window_index, slice)| {
            println!("Window {}/19", window_index + 1);
            expand_recursively(
                (slice[0], slice[1]),
                rules,
                &mut freq_map,
                depth,
                0,
            )
        });
        freq_map
    }

    fn expand_recursively(
        pair: (char, char),
        rules: &Rules,
        freq_map: &mut FreqMap,
        max_depth: usize,
        depth: usize,
    ) {
        if depth < max_depth {
            let (l, r) = *rules.get(&pair).unwrap();
            update_freq_map(freq_map, &[l.0, l.1, r.0, r.1]);
            expand_recursively(l, rules, freq_map, max_depth, depth + 1);
            expand_recursively(r, rules, freq_map, max_depth, depth + 1);
        }
    }

    fn update_freq_map(freq_map: &mut FreqMap, chars: &[char]) {
        chars
            .iter()
            .for_each(|c| *freq_map.entry(*c).or_default() += 1);
    }
}

#[cfg(test)]
mod test {
    use crate::naive::string_polymer_rules;

    const WEBSITE_EXAMPLE: &str = "NNCB\n\nCH -> B\nHH -> N\nCB -> H\nNH -> C\nHB -> C\nHC -> B\nHN -> C\nNN -> C\nBH -> H\nNC -> B\nNB -> B\nBN -> B\nBB -> N\nBC -> B\nCC -> N\nCN -> C\n";

    #[test]
    fn expansion() {
        let (mut polymer, rules) = string_polymer_rules(WEBSITE_EXAMPLE);
        polymer.pair_insertion(&rules);
        assert_eq!(&polymer.to_string(), "NCNBCHB",);
    }
}
