mod utils;

use crate::utils::csv_reader;

fn main() {
    let data = csv_reader::read_csv("data/fx_data.csv");

    println!("{:?}", data);
}




