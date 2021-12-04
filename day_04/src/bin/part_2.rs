use day_04::input_game;

fn main() {
    let (balls, mut game) = input_game();
    let game_sim = || {
        let mut num_winners = 0;
        let mut last_and_lowest_score;
        for ball in balls {
            game.draw(ball);
            if let Some(winners) = game.winners() {
                println!("Player(s) {:?} won on ball {}", &winners, ball);
                num_winners += winners.len();
                last_and_lowest_score = winners
                    .iter()
                    .map(|player_index| game.score_of(*player_index))
                    .min()
                    // Safe because game.winners() returns None instead of an empty Vec
                    .unwrap();
                if num_winners == game.players() {
                    return last_and_lowest_score;
                }
            }
        }
        unreachable!(
            "It is genuinely not possible to only just win on the last ball"
        );
    };
    let score = game_sim();
    println!("\nThe last winner's score is: {}", score);
}
