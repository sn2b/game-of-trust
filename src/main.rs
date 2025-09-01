fn main() {
    if let Err(e) = game_of_trust::run_tournament() {
        eprintln!("Error running tournament: {}", e);
        std::process::exit(1);
    }
}
