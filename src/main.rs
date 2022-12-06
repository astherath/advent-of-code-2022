pub mod common;
fn main() {
    println!(
        "file data: {:#?}",
        common::read_input("01/input.txt").expect("error opening file")
    );
}
