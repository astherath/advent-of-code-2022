use common::read_input;
const DIR_PATH: &'static str = "test_dir.txt";
// const DIR_PATH: &'static str = "directions.txt";
// const STACK_PATH: &'static str = "stack.txt";
const STACK_PATH: &'static str = "test_stack.txt";
fn main() {
    let directions_strings = read_input(DIR_PATH)
        .unwrap()
        .into_iter()
        .filter(|x| !x.is_empty())
        .collect::<Vec<String>>();

    let stacks = get_stacks();

    let directions: Vec<Direction> = directions_strings
        .into_iter()
        .map(|x| Direction::from(&x))
        .collect();

    dbg!(&stacks);
    let mut crane = Crane::new(stacks);
    for direction in directions {
        crane.move_crate(direction);
    }
    let stacks_s = crane.inner();
    dbg!(&stacks_s);
}

struct Crane {
    stacks: Vec<Stack>,
}

impl Crane {
    fn new(stacks: Vec<Stack>) -> Self {
        Self {
            stacks
        }
    }

    fn move_crate(&mut self, direction: Direction) {
        for _ in 0..direction.amount {
            let mut c_crate = self.stacks[direction.from].pop();
            c_crate.swap(direction.to);
            self.stacks[direction.to].push(c_crate);
        }
    }

    fn inner(self) -> Vec<Stack> {
        self.stacks
    }
}

#[derive(Debug)]
struct Direction {
    amount: usize,
    from: usize,
    to: usize,
}

impl Direction {
    fn from(line: &str) -> Self {
        let mut nums = line
            .chars()
            .filter(|x| x.is_alphanumeric())
            .map(|x| x.to_string().parse::<usize>().unwrap());
        let (amount, from, to) = (
            nums.next().unwrap(),
            nums.next().unwrap(),
            nums.next().unwrap(),
        );
        Self { amount, from, to }
    }
}

#[derive(Debug)]
struct Crate {
    index: usize,
    value: char,
}

impl Crate {
    fn many_from(line: &str, index: usize) -> Vec<Self> {
        line.chars().map(|x| Self { index, value: x }).collect()
    }
    fn swap(&mut self, to: usize) {
        self.index = to;
    }
}

#[derive(Debug)]
struct Stack {
    index: usize,
    crates: Vec<Crate>,
}

impl Stack {
    fn from(line: String, index: usize) -> Self {
        let crates = Crate::many_from(&line, index);
        Self { index, crates }
    }

    fn pop(&mut self) -> Crate {
        self.crates.pop().unwrap()
    }

    fn push(&mut self, c_crate: Crate) {
        self.crates.push(c_crate);
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

    read_input(STACK_PATH)
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
