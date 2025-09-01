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

    let html = format!(r#"<!doctype html>
<html lang=\"en\">
<head>
<meta charset=\"utf-8\">
<meta name=\"viewport\" content=\"width=device-width,initial-scale=1\">
<title>Game of Trust — Tournament</title>
<style>
  :root{{ --accent:#0366d6; --muted:#666; --card:#f6f8fa; }}
  body {{ font-family: system-ui, -apple-system, 'Segoe UI', Roboto, 'Helvetica Neue', Arial; padding: 2rem; max-width: 900px; margin: auto; color: #111; background: #fff; }}
  header {{ display:flex; flex-direction:column; gap:.5rem; margin-bottom:1rem }}
  .topline {{ display:flex; justify-content:space-between; align-items:center; gap:1rem }}
  h1 {{ margin:0; font-size:1.5rem }}
  .meta {{ color: var(--muted); font-size:.95rem }}
  .cards {{ display:flex; gap:1rem; margin-top:.6rem; flex-wrap:wrap }}
  .card {{ background:var(--card); padding:.6rem .8rem; border-radius:10px; box-shadow: 0 1px 2px rgba(0,0,0,0.03); display:flex; gap:.6rem; align-items:center }}
  .badge {{ background:var(--accent); color:white; padding:.25rem .5rem; border-radius:6px; font-weight:600; font-size:.85rem }}
  .card .big {{ font-weight:700; font-size:1.05rem }}
  h2 {{ margin-top:1.2rem; color:#222 }}
  table {{ width: 100%; border-collapse: collapse; margin-top: .6rem; background: white; }}
  th, td {{ text-align: left; padding: .6rem; border-bottom: 1px solid #eee; }}
  thead th {{ background: #fafafa; font-weight:700; }}
  tr.top-row td {{ background: linear-gradient(90deg,#fff9e6,#fff); font-weight:700; }}
  .footer {{ margin-top: 1.2rem; color: var(--muted); font-size: .9rem; }}
  @media (max-width:600px){{ .cards{{flex-direction:column}} table{{font-size:.95rem}} }}
</style>
</head>
<body>
<header>
  <div class=\"topline\">
    <div>
      <h1>Game of Trust</h1>
      <div class=\"meta\">Last tournament: <span id=\"tournament-date\">—</span></div>
    </div>
    <div class=\"cards\">
      <div class=\"card\">
        <div class=\"badge\">Highest ever</div>
        <div style=\"display:flex;flex-direction:column; margin-left:.4rem\">
          <div class=\"big\" id=\"highest-player\">{highest_player}</div>
          <div class=\"meta\"><span id=\"highest-score\">{highest_score:.3}</span> • <span id=\"highest-date\">—</span></div>
        </div>
      </div>
    </div>
  </div>
</header>

<h2>Leaderboard (normalized)</h2>
<table id=\"leaderboard\" aria-describedby=\"leaderboard\">
  <thead><tr><th style=\"width:56px\">Rank</th><th>Strategy</th><th style=\"text-align:right;\">Score</th></tr></thead>
  <tbody>
{rows}
  </tbody>
</table>

<div class=\"footer\">This page is auto-generated. View the full history at <code>/data/history.json</code>.</div>

<script>
// format dates in client local timezone
(function(){
  try {
    var ts = {last_ts};
    var highestTs = {highest_ts};
    if (ts && ts > 0) document.getElementById('tournament-date').textContent = new Date(ts * 1000).toLocaleString();
    if (highestTs && highestTs > 0) document.getElementById('highest-date').textContent = new Date(highestTs * 1000).toLocaleString();
  } catch(e){}
})();
</script>
</body>
</html>"#, highest_player=highest_player, highest_score=highest, rows=rows, last_ts=last_ts, highest_ts=highest_ts);

    html
}
