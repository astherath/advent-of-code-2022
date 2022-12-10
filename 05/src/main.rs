use common::read_input;
fn main() {
    let directions = read_input("directions.txt")
        .unwrap()
        .into_iter()
        .filter(|x| !x.is_empty())
        .collect::<Vec<String>>();

    let stacks = get_stacks();
    dbg!(&stacks);

    for direction in directions {}
}

#[derive (Debug)]
struct Crate {
    index: usize,
    value: char,
}

impl Crate {
    fn many_from(line: &str, index: usize) -> Vec<Self> {
        line.chars().map(|x| Self {
            index,
            value: x,
        }).collect()
    }
}


#[derive (Debug)]
struct Stack {
    index: usize,
    crates: Vec<Crate>,
}

impl Stack {
    fn from(line: String, index: usize) -> Self {
        let crates = Crate::many_from(&line, index);
        Self { index, crates }
    }
}

fn get_stacks() -> Vec<Stack> {
    get_stack_str()
        .into_iter()
        .enumerate()
        .map(|(i, line)| Stack::from(line, i))
        .collect()
}

fn get_stack_str() -> Vec<String> {
    const MARKER: char = '0';
    const BUCKET_SIZE: usize = 9;

    let mut stack_raw: Vec<Vec<char>> = vec![];

    read_input("stack.txt")
        .unwrap()
        .into_iter()
        .filter(|x| !x.is_empty())
        .collect::<Vec<String>>()
        .into_iter()
        .for_each(|line| {
            if line.contains("[") {
                stack_raw.push(
                    line.replace("[", "")
                        .replace("]", "")
                        .replace(" ", "X")
                        .replace("XXXX", &MARKER.to_string())
                        .replace("X", "")
                        .chars()
                        .collect(),
                );
            }
        });

    let mut stack_sorter = vec!["".to_string(); BUCKET_SIZE];
    stack_raw.into_iter().for_each(|stack| {
        stack
            .into_iter()
            .map(|x| x.to_string())
            .enumerate()
            .for_each(|(i, val)| {
                if val != MARKER.to_string() {
                    stack_sorter[i].push_str(&val);
                }
            });
    });

    stack_sorter
        .into_iter()
        .map(|x| x.chars().rev().collect())
        .collect()
}
