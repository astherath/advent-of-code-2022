#!/usr/bin/env python
import fire
import os

def create(day: int) -> None:
    """creates a new dir for the AOC day and puts in input too."""
    day_str = ""
    match len(str(day)):
        case 1:
            day_str = f"0{day}"
        case 2:
            day_str = str(day)
    os.system(f"mkdir {day_str}")
    os.system(f"mv 'input.txt' {day_str}")

    os.system(f"cd {day_str}")

if __name__ == '__main__':
    fire.Fire(create)
