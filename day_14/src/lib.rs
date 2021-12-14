pub mod naive {
    use std::collections::HashMap;
    use std::fs;

    pub type Rules = HashMap<(char, char), [char; 3]>;

    pub fn input_polymer_rules() -> (Polymer, Rules) {
        let input = fs::read_to_string("day_14/input")
            .expect("Failed to read input file");
        string_polymer_rules(&input)
    }

    fn string_polymer_rules(s: &str) -> (Polymer, Rules) {
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

    #[cfg(test)]
    mod test {
        use super::string_polymer_rules;

        const WEBSITE_EXAMPLE: &str = "NNCB\n\nCH -> B\nHH -> N\nCB -> H\nNH -> C\nHB -> C\nHC -> B\nHN -> C\nNN -> C\nBH -> H\nNC -> B\nNB -> B\nBN -> B\nBB -> N\nBC -> B\nCC -> N\nCN -> C\n";

        #[test]
        fn expansion() {
            let (mut polymer, rules) = string_polymer_rules(WEBSITE_EXAMPLE);
            polymer.pair_insertion(&rules);
            assert_eq!(&polymer.to_string(), "NCNBCHB",);
        }
    }
}

pub mod smart {
    use std::collections::HashMap;
    use std::fs;

    pub type Rules = HashMap<(char, char), ((char, char), (char, char))>;
    pub type FreqMap<T> = HashMap<T, usize>;

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

    fn build_rule_freq_map(chars: &[char]) -> FreqMap<(char, char)> {
        let mut map = HashMap::new();
        chars.windows(2).for_each(|slice| {
            *map.entry((slice[0], slice[1])).or_default() += 1
        });
        map
    }

    fn build_char_freq_map(
        start: &[char],
        rule_freq_map: FreqMap<(char, char)>,
    ) -> FreqMap<char> {
        let mut map = HashMap::new();
        rule_freq_map.into_iter().for_each(|(pair, freq)| {
            *map.entry(pair.0).or_default() += freq;
            //*map.entry(pair.1).or_default() += freq;
        });
        *map.entry(*start.last().unwrap()).or_default() += 1;
        map
    }

    pub fn freq_map(
        start: &[char],
        rules: &Rules,
        depth: usize,
    ) -> FreqMap<char> {
        let initial_map = build_rule_freq_map(start);
        let rule_freq_map =
            (0..depth)
                .into_iter()
                .fold(initial_map, |rule_freq_map, _| {
                    let mut next_map =
                        HashMap::with_capacity(rule_freq_map.len());
                    rule_freq_map.into_iter().for_each(|(rule, freq)| {
                        let (l, r) = *rules.get(&rule).unwrap();
                        *next_map.entry(l).or_default() += freq;
                        *next_map.entry(r).or_default() += freq;
                    });
                    next_map
                });
        build_char_freq_map(start, rule_freq_map)
    }

    #[cfg(test)]
    mod test {
        use super::*;

        const WEBSITE_EXAMPLE: &str = "NNCB\n\nCH -> B\nHH -> N\nCB -> H\nNH -> C\nHB -> C\nHC -> B\nHN -> C\nNN -> C\nBH -> H\nNC -> B\nNB -> B\nBN -> B\nBB -> N\nBC -> B\nCC -> N\nCN -> C\n";
        #[test]
        fn rule_freq_map_creation() {
            let (chars, _) = string_chars_rules(WEBSITE_EXAMPLE);
            let rule_freq_map = build_rule_freq_map(&chars);

            assert_eq!(rule_freq_map.get(&('N', 'N')), Some(&1));
            assert_eq!(rule_freq_map.get(&('N', 'C')), Some(&1));
            assert_eq!(rule_freq_map.get(&('C', 'B')), Some(&1));
            assert_eq!(rule_freq_map.len(), 3);
        }

        #[test]
        fn char_freq_map_creation() {
            let (chars, _) = string_chars_rules(WEBSITE_EXAMPLE);
            let rule_freq_map = build_rule_freq_map(&chars);
            println!("{:?}", &rule_freq_map);
            let char_freq_map = build_char_freq_map(&chars, rule_freq_map);

            assert_eq!(char_freq_map.get(&'N'), Some(&2), "Miscounted Ns");
            assert_eq!(char_freq_map.get(&'C'), Some(&1), "Miscounted Cs");
            assert_eq!(char_freq_map.get(&'B'), Some(&1), "Miscounted Bs");
            assert_eq!(char_freq_map.len(), 3, "Extraneous junk");
        }

        #[test]
        fn expansion() {
            let (chars, rules) = string_chars_rules(WEBSITE_EXAMPLE);
            let char_freq_map = freq_map(&chars, &rules, 1);
            println!("{:?}", &char_freq_map);

            assert_eq!(char_freq_map.get(&'N'), Some(&2));
            assert_eq!(char_freq_map.get(&'C'), Some(&2));
            assert_eq!(char_freq_map.get(&'B'), Some(&2));
            assert_eq!(char_freq_map.get(&'H'), Some(&1));
            assert_eq!(char_freq_map.len(), 4);
        }
    }
}
