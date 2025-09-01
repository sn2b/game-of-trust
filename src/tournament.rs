use crate::game::{Action, payoff};
use crate::strategies::get_factories;
use serde::{Serialize, Deserialize};
use rand::Rng;
use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerScore {
    pub name: String,
    pub total_normalized: f64,
    pub total_raw: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MatchRecord {
    pub player_a: String,
    pub player_b: String,
    pub rounds: usize,
    pub a_raw: i32,
    pub b_raw: i32,
    pub a_normalized: f64,
    pub b_normalized: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TournamentResult {
    pub timestamp: u64,
    pub leaderboard: Vec<PlayerScore>,
    pub matches: Vec<MatchRecord>,
}

pub fn run_tournament() -> Result<(), Box<dyn std::error::Error>> {
    let factories = get_factories();
    let mut totals: HashMap<String, f64> = HashMap::new();
    for f in &factories {
        totals.insert(f.name.to_string(), 0.0);
    }

    let mut matches: Vec<MatchRecord> = Vec::new();
    let mut rng = rand::thread_rng();

    for i in 0..factories.len() {
        for j in i..factories.len() {
            let rounds = rng.gen_range(150..=250);
            let mut a = (factories[i].make)();
            let mut b = (factories[j].make)();
            let mut hist_a: Vec<Action> = Vec::new();
            let mut hist_b: Vec<Action> = Vec::new();
            let mut score_a: i32 = 0;
            let mut score_b: i32 = 0;

            for _ in 0..rounds {
                let ma = a.next_move(&hist_a, &hist_b);
                let mb = b.next_move(&hist_b, &hist_a);
                let (pa, pb) = payoff(ma, mb);
                score_a += pa;
                score_b += pb;
                hist_a.push(ma);
                hist_b.push(mb);
            }

            let a_norm = (score_a as f64) * 200.0 / (rounds as f64);
            let b_norm = (score_b as f64) * 200.0 / (rounds as f64);

            *totals.get_mut(factories[i].name).unwrap() += a_norm;
            *totals.get_mut(factories[j].name).unwrap() += b_norm;

            matches.push(MatchRecord {
                player_a: factories[i].name.to_string(),
                player_b: factories[j].name.to_string(),
                rounds,
                a_raw: score_a,
                b_raw: score_b,
                a_normalized: a_norm,
                b_normalized: b_norm,
            });
        }
    }

    // build leaderboard
    let mut leaderboard: Vec<PlayerScore> = totals.into_iter().map(|(name, total)| PlayerScore {
        name,
        total_normalized: total,
        total_raw: 0.0,
    }).collect();

    leaderboard.sort_by(|a, b| b.total_normalized.partial_cmp(&a.total_normalized).unwrap());

    let result = TournamentResult {
        timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
        leaderboard: leaderboard.clone(),
        matches: matches.clone(),
    };

    // ensure data dir exists and write history
    create_dir_all("data")?;
    let history_path = Path::new("data/history.json");
    let mut history: Vec<TournamentResult> = if history_path.exists() {
        let mut s = String::new();
        File::open(history_path)?.read_to_string(&mut s)?;
        serde_json::from_str(&s).unwrap_or_default()
    } else {
        Vec::new()
    };

    history.push(result.clone());
    let mut file = File::create(history_path)?;
    file.write_all(serde_json::to_string_pretty(&history)?.as_bytes())?;

    // generate index html and write both static and docs (rendering outsourced to output module)
    let html = crate::output::render_index_html(&leaderboard, &history);

    create_dir_all("static")?;
    let mut static_file = File::create("static/index.html")?;
    static_file.write_all(html.as_bytes())?;

    create_dir_all("docs/data")?;
    let mut docs_file = File::create("docs/index.html")?;
    docs_file.write_all(html.as_bytes())?;
    let mut docs_data_file = File::create("docs/data/history.json")?;
    docs_data_file.write_all(serde_json::to_string_pretty(&history)?.as_bytes())?;

    println!("Tournament complete. Results written to data/history.json, static/index.html and docs/ (ready for Pages)");
    Ok(())
}
