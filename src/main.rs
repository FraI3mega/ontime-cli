mod stops;

use std::collections::HashMap;

use color_eyre::Result;
use comfy_table::Table;
use time::OffsetDateTime;

use crate::stops::{get_stop_data, get_stops, select_stop, sitemap::get_sitemap};

fn main() -> Result<()> {
    color_eyre::install()?;
    let sitemap = get_sitemap("https://dip.mzkopole.pl/".to_string())?;
    dbg!(sitemap.clone());

    let stops: HashMap<String, u16> = get_stops(sitemap.clone())?;
    let stop_number = select_stop(stops)?;

    let stop_data = get_stop_data(sitemap, stop_number)?;
    let mut table = Table::new();
    table.set_header(vec!["Line number", "Line name", "Arrives in"]);

    for bus in stop_data.departures {
        let departs_in = OffsetDateTime::from_unix_timestamp(bus.real_departure / 1000)?
            - OffsetDateTime::now_utc();

        table.add_row(vec![
            bus.line_name,
            bus.direction_name,
            format!("{}m", departs_in.whole_minutes().to_string()),
        ]);
    }

    println!("\n{}", table);
    Ok(())
}
