use calamine::{open_workbook, Xls, Xlsx};

fn main() {
    // let path = "./file_example_XLS_10(1) (1).xls".to_string();
    let path = "./spreadsheet.xls".to_string();
    let workbook: Xls<_> = open_workbook(path).expect("Cannot open file");
    println!("what");
}
