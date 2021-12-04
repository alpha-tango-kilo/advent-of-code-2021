use day_04::input_game;

fn main() {
    let (balls, mut game) = input_game();
    let game_sim = || {
        for ball in balls {
            game.draw(ball);
            if let Some(index) = game.winner() {
                return index;
            }
        }
        unreachable!("Balls exhausted with no winner");
    };
    let winner = game_sim();
    let score = game.score_of(winner);
    println!("Winner's final score: {}", score);
}
