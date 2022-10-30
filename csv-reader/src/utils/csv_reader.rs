use csv::ReaderBuilder;
use serde::Deserialize;
use std::fs::File;

fn get_csv_path(path: &str) -> std::path::PathBuf {
    let project_path = env!("CARGO_MANIFEST_DIR");

    std::path::Path::new(project_path)
        .parent()
        .unwrap()
        .join(path)
}

pub fn read_csv(path: &str) -> anyhow::Result<Vec<FxData>> {
    let csv_path = get_csv_path(path);
    let file = File::open(csv_path)?;
    
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let mut fx_data = Vec::new();

    for result in rdr.deserialize() {
        let record: FxData = result.unwrap();
        fx_data.push(record);
    }

    Ok(fx_data)
}

#[derive(Debug, Deserialize)]
pub struct FxData {
    pub trade_name: String,
    pub trade_type: String,
    pub trading_date: chrono::NaiveDate,
    pub prior_day_settlement_price: f32,
    pub open_price: f32,
    pub high_price: f32,
    pub low_price: f32,
    pub close_price: f32,
    pub settlement_price: f32,
    pub on_the_day: f32,
    pub daily_comparsion: Option<i32>,
    pub swap_point: Option<i32>,
    pub trading_volume: i32,
    pub open_interest_volume: i32
}