use crate::tournament::{PlayerScore, TournamentResult};

pub fn render_index_html(leaderboard: &Vec<PlayerScore>, history: &Vec<TournamentResult>) -> String {
    // Last tournament timestamp (0 if none)
    let last_ts = history.last().map(|t| t.timestamp).unwrap_or(0);

    // Highest ever across history
    let mut highest = std::f64::NEG_INFINITY;
    let mut highest_player = String::from("—");
    let mut highest_ts: u64 = 0;
    for t in history {
        for p in &t.leaderboard {
            if p.total_normalized > highest {
                highest = p.total_normalized;
                highest_player = p.name.clone();
                highest_ts = t.timestamp;
            }
        }
    }
    if highest == std::f64::NEG_INFINITY {
        highest = 0.0;
        highest_player = String::from("—");
        highest_ts = 0;
    }

    // rows for leaderboard
    let mut rows = String::new();
    for (i, p) in leaderboard.iter().enumerate() {
        if i == 0 {
            rows.push_str(&format!("<tr class=\"top-row\"><td>{}</td><td>{}</td><td style=\"text-align:right\">{:.3}</td></tr>\n", i+1, p.name, p.total_normalized));
        } else {
            rows.push_str(&format!("<tr><td>{}</td><td>{}</td><td style=\"text-align:right\">{:.3}</td></tr>\n", i+1, p.name, p.total_normalized));
        }
    }

    // Build HTML by concatenating parts to avoid format! parsing of braces inside CSS/JS
    let mut html = String::new();

    html.push_str("<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n<meta name=\"viewport\" content=\"width=device-width,initial-scale=1\">\n<title>Game of Trust — Tournament</title>\n<style>\n  :root{ --accent:#0366d6; --muted:#666; --card:#f6f8fa; }\n  body { font-family: system-ui, -apple-system, 'Segoe UI', Roboto, 'Helvetica Neue', Arial; padding: 2rem; max-width: 900px; margin: auto; color: #111; background: #fff; }\n  header { display:flex; flex-direction:column; gap:.5rem; margin-bottom:1rem }\n  .topline { display:flex; justify-content:space-between; align-items:center; gap:1rem }\n  h1 { margin:0; font-size:1.5rem }\n  .meta { color: var(--muted); font-size:.95rem }\n  .cards { display:flex; gap:1rem; margin-top:.6rem; flex-wrap:wrap }\n  .card { background:var(--card); padding:.6rem .8rem; border-radius:10px; box-shadow: 0 1px 2px rgba(0,0,0,0.03); display:flex; gap:.6rem; align-items:center }\n  .badge { background:var(--accent); color:white; padding:.25rem .5rem; border-radius:6px; font-weight:600; font-size:.85rem }\n  .card .big { font-weight:700; font-size:1.05rem }\n  h2 { margin-top:1.2rem; color:#222 }\n  table { width: 100%; border-collapse: collapse; margin-top: .6rem; background: white; }\n  th, td { text-align: left; padding: .6rem; border-bottom: 1px solid #eee; }\n  thead th { background: #fafafa; font-weight:700; }\n  tr.top-row td { background: linear-gradient(90deg,#fff9e6,#fff); font-weight:700; }\n  .footer { margin-top: 1.2rem; color: var(--muted); font-size: .9rem; }\n  @media (max-width:600px){ .cards{flex-direction:column} table{font-size:.95rem} }\n</style>\n</head>\n<body>\n<header>\n  <div class=\"topline\">\n    <div>\n      <h1>Game of Trust</h1>\n      <div class=\"meta\">Last tournament: <span id=\"tournament-date\">—</span></div>\n    </div>\n    <div class=\"cards\">\n      <div class=\"card\">\n        <div class=\"badge\">Highest ever</div>\n        <div style=\"display:flex;flex-direction:column; margin-left:.4rem\">\n          <div class=\"big\" id=\"highest-player\">");

    // highest player name
    html.push_str(&highest_player);

    html.push_str("</div>\n          <div class=\"meta\"><span id=\"highest-score\">", );
    // highest score formatted
    html.push_str(&format!("{:.3}", highest));
    html.push_str("</span> • <span id=\"highest-date\">—</span></div>\n        </div>\n      </div>\n    </div>\n  </div>\n</header>\n\n<h2>Leaderboard (normalized)</h2>\n<table id=\"leaderboard\" aria-describedby=\"leaderboard\">\n  <thead><tr><th style=\"width:56px\">Rank</th><th>Strategy</th><th style=\"text-align:right;\">Score</th></tr></thead>\n  <tbody>\n");

    // insert rows
    html.push_str(&rows);

    html.push_str("  </tbody>\n</table>\n\n<div class=\"footer\">This page is auto-generated. View the full history at <code>/data/history.json</code>.</div>\n\n<script>\n// format dates in client local timezone\n(function(){\n  try {\n    var ts = ");

    html.push_str(&last_ts.to_string());

    html.push_str(";\n    var highestTs = ");

    html.push_str(&highest_ts.to_string());

    html.push_str(";\n    if (ts && ts > 0) document.getElementById('tournament-date').textContent = new Date(ts * 1000).toLocaleString();\n    if (highestTs && highestTs > 0) document.getElementById('highest-date').textContent = new Date(highestTs * 1000).toLocaleString();\n  } catch(e){}\n})();\n</script>\n</body>\n</html>");

    html
}
