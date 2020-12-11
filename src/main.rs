pub mod reader;
pub mod matrix;
pub mod tree;
pub mod forest;

use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filetype: u8 = input_file_type(&args[1]);
    let data = match filetype {
        1 => reader::read_csv(&args[1]),
        _ => panic!("Input filetype not implemented!"),
    };
    let f = forest::Forest::grow(data, 10, 0.333, 5, 0.666, 4);
    f.var_importance();
    //for tree in f.trees {
        //tree.print(&9999, &"SIDE")
        
    //}
}

fn input_file_type(filename: &str) -> u8 {
    let file_split: Vec<_> = filename.split(".").map(|s| s).collect();
    let suffix = file_split[file_split.len() - 1];
    match suffix {
        "csv" => 1,
        "tsv" => 2,
        "gz" => 9,
        _ => 0,
    }
}

