use common::read_input;
fn main() {
    let path = "input.txt";
    let lines = read_input(path).expect("couldnt open file");
    let mut counter = 0;
    let mut nums = vec![];
    for line in lines {
        if line.len() == 0 {
            nums.push(counter);
            counter = 0;
        } else {
            counter += line.parse::<usize>().expect("couldn't parse");
        }
    }

    nums.sort();
    let max: usize = nums.into_iter().rev().take(3).sum();

    println!("max: {}", max);
}
