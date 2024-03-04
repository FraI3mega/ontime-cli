use std::{collections::HashMap, io::Cursor};

use color_eyre::eyre::Result;
use serde::Deserialize;
use skim::prelude::*;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StopData {
    _stop_point_symbol: String,
    _stop_point_id: u16,
    _stop_point_name: String,
    _response_date: usize,
    pub departures: Vec<BusData>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BusData {
    _course_id: usize,
    _scheduled_departure_sec: u32,
    _scheduled_departure: usize,
    pub real_departure: i64, // NOTE: this in in miliseconds
    _vehicle_id: String,
    _variant_id: u32,
    _order_in_course: u16,
    _passed: bool,
    _lack: bool,
    _on_stop_point: bool,
    pub line_name: String,
    pub direction_name: String,
}

pub fn get_stop_data(main_url: String, stop_number: u16) -> Result<StopData> {
    let data: StopData = ureq::get(
        format!(
            "{}getRealtime.json?stopPointSymbol={}",
            main_url, stop_number
        )
        .as_str(),
    )
    .call()?
    .into_json()?;

    Ok(data)
}

pub fn select_stop(stops: HashMap<String, u16>) -> Result<u16> {
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(true)
        .build()
        .unwrap();

    let input = stops
        .keys()
        .map(|s| s.as_str())
        .collect::<Vec<&str>>()
        .join("\n");

    // `SkimItemReader` is a helper to turn any `BufRead` into a stream of `SkimItem`
    // `SkimItem` was implemented for `AsRef<str>` by default
    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input));

    // `run_with` would read and show items from the stream
    let selected_items = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());
    Ok(*stops.get(&selected_items[0].output().to_string()).unwrap())
}

fn get_stops(url: String) {
    todo!("Not done")
}