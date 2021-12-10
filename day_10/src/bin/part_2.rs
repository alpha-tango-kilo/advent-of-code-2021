use day_10::*;
use std::fs;

fn main() {
    let mut scores = fs::read_to_string("day_10/input")
        .expect("Failed to read input file")
        .lines()
        .filter_map(|line| {
            use ParseResult::*;
            match parse_line(line) {
                Ok | Corrupt(_) => None,
                Incomplete(n) => Some(n),
            }
        })
        .collect::<Vec<_>>();
    scores.sort_unstable();
    assert_eq!(
        scores.len() % 2,
        1,
        "Prompt said there'd be an odd number of scores"
    );
    let middle = scores[scores.len() / 2];
    println!("The middle score of the syntax errors is: {}", middle);
}

#[cfg(test)]
mod tests {
    use day_10::*;

    const WEBSITE_EXAMPLE: &str = "\
        [({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]
    ";

    #[test]
    fn example() {
        let mut scores = WEBSITE_EXAMPLE
            .lines()
            .filter_map(|line| {
                use ParseResult::*;
                match parse_line(line.trim()) {
                    Ok | Corrupt(_) => None,
                    Incomplete(n) => Some(n),
                }
            })
            .collect::<Vec<_>>();
        scores.sort_unstable();
        println!("Scores: {:?}", &scores);
        assert_eq!(
            scores.len() % 2,
            1,
            "Prompt said there'd be an odd number of scores"
        );
        let middle = scores[scores.len() / 2];
        assert_eq!(middle, 288957);
    }
}
