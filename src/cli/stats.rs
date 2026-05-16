use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;

use crate::paths;
use crate::storage::db;
use crate::storage::stats::summary;

use super::args::StatsArgs;

pub fn run(args: StatsArgs) -> Result<()> {
    let conn = db::open(&paths::db_path())?;
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;

    let since = match args.period.as_str() {
        "today" => now - 86400,
        "week" => now - 7 * 86400,
        "month" => now - 30 * 86400,
        _ => 0,
    };

    let (words, avg_wpm, peak_wpm, sessions) = summary(&conn, since)?;

    println!("Period: {}", args.period);
    println!("{}", "-".repeat(40));
    println!("  Total words read : {}", words);
    println!("  Avg WPM          : {:.0}", avg_wpm);
    println!("  Peak WPM         : {}", peak_wpm);
    println!("  Sessions         : {}", sessions);

    Ok(())
}
