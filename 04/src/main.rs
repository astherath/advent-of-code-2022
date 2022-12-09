use common::read_input;

type TupleRange = (usize, usize);

fn main() {
        let lines = read_input("input.txt")
        .unwrap()
        .into_iter()
        .filter(|x| !x.is_empty());

    let mut counter = 0;

    for line in lines {
        let (left, right) = {
            let mut pair = line.split(",");
            let _left = {
                let mut l_pair = pair
                    .next()
                    .unwrap()
                    .split("-")
                    .map(|x| x.parse::<usize>().unwrap());
                (l_pair.next().unwrap(), l_pair.next().unwrap())
            };
            let _right = {
                let mut r_pair = pair
                    .next()
                    .unwrap()
                    .split("-")
                    .map(|x| x.parse::<usize>().unwrap());
                (r_pair.next().unwrap(), r_pair.next().unwrap())
            };

            (_left, _right)
        };

        let is_contained = |pairs: Vec<TupleRange>| -> bool {
            let (left, right) = (pairs[0], pairs[1]);
            let result = (left.0 >= right.0 && left.0 <= right.1)
                || (left.1 >= right.0 && left.1 <= right.1);
            result
        };

        if is_contained(vec![left, right]) || is_contained(vec![right, left]) {
            counter += 1;
        }
    }

    println!("counter: {}", counter);
}
