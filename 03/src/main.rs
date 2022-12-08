use common::read_input;
use std::collections::HashSet;

fn main() {
    let lines = read_input("input.txt").unwrap();
    let mut inters: Vec<char> = vec![];

    let binding = lines
        .into_iter()
        .filter(|x| !x.is_empty())
        .collect::<Vec<String>>();
    let f_lines = binding.chunks(3);

    for inner_lines in f_lines {
        dbg!(inner_lines);
        let (s1, s2, s3) = match &inner_lines {
            &[first, sec, third] => (first, sec, third),
            _ => unreachable!(),
        };

        let set_1 = s1.chars().fold(HashSet::new(), |mut acc, x| {
            acc.insert(x);
            acc
        });

        let set_2 = s2.chars().fold(HashSet::new(), |mut acc, x| {
            acc.insert(x);
            acc
        });

        let set_3 = s3.chars().fold(HashSet::new(), |mut acc, x| {
            acc.insert(x);
            acc
        });

        let intersection = set_1
            .intersection(&set_2)
            .map(|x| x.clone())
            .fold(HashSet::new(), |mut acc, x| {
                acc.insert(x);
                acc
            })
            .intersection(&set_3)
            .map(|x| x.clone())
            .nth(0)
            .unwrap();
        inters.push(intersection.clone());
    }

    let total: usize = inters.into_iter().map(|x| get_letter_val(x)).sum();
    println!("total: {}", total);
}

fn get_letter_val(val: char) -> usize {
    if val.is_lowercase() {
        (val as usize) - 96
    } else {
        (val as usize) - 38
    }
}
