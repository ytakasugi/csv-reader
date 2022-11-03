mod utils;

use crate::utils::csv_reader;
use crate::utils::calc;

fn main() {
    let data = csv_reader::read_csv("data/fx_data4.csv").unwrap();
    let average = calc::moving_average(data, 7);

    println!("{:?}", average);
}




