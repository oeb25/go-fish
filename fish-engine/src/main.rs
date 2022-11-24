use fish_engine::{strategy::Strat, Game};

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();
    log::info!("Starting out...");

    let mut counts = [0; 6];

    let count = 10000;
    let before = std::time::Instant::now();
    for iter in 0..count {
        let mut strats = [
            Strat::random(),
            Strat::random(),
            Strat::random(),
            Strat::random(),
            Strat::random(),
            Strat::random(),
        ];
        fastrand::shuffle(&mut strats);
        let mut game = Game::new(5, strats);

        while !game.stage.is_done() {
            game.step();
        }

        let (winner, _) = game
            .players
            .iter()
            .enumerate()
            .max_by_key(|(_, p)| p.books.iter().count())
            .unwrap();

        eprintln!("{}/{count}", iter + 1);
        // println!("{winner}");

        counts[winner] += 1;
    }

    eprintln!("{:?}", counts);

    eprintln!(
        "{}µs/sample",
        before.elapsed().as_micros() as f32 / count as f32
    );
}
