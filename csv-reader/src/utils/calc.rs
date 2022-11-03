use crate::utils::csv_reader::FxData;

pub fn moving_average(data: Vec<FxData>, prev: usize) -> f32 {
    let end = data.len();
    let start = end - prev;
    let list = &data[start..end];

    let mut sum: f32 = 0.;

    for price in list.iter() {
        sum += price.close_price 
    }

    sum / prev as f32
}