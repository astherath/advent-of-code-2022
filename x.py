#!/usr/bin/env python
import fire
import os

def create(day: int) -> None:
    """creates a new dir for the AOC day and puts in input too."""
    day_str = ""
    match len(day):
        case 1:
            day_str = f"0{day}"
        case 2:
            day_str = str(day)
    os.system(f"mkdir {day_str}")

    input_url = f"https://adventofcode.com/2022/day/{day}/input"
    os.system(f'wget "{input_url}" "input.txt"')

    os.system(f"chdir {day_str}")

if __name__ == '__main__':
    fire.Fire(create)
