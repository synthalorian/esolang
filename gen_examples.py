#!/usr/bin/env python3
import sys

def place_on_grid(width, height, instructions):
    dirs = [(2, 1), (1, 2), (-2, 1), (1, -2)]
    grid = [['.' for _ in range(width)] for _ in range(height)]
    x, y = 0, 0
    d = 0
    grid[y][x] = 'S'

    for i, ch in enumerate(instructions):
        dx, dy = dirs[d]
        x = (x + dx) % width
        y = (y + dy) % height
        if grid[y][x] != '.':
            print(f"Collision at ({x},{y}) step {i} char '{ch}' existing '{grid[y][x]}'", file=sys.stderr)
            sys.exit(1)
        grid[y][x] = ch

    return grid

def grid_to_str(grid):
    return '\n'.join(''.join(row) for row in grid)

def build_hello():
    seq = (
        "25*"
        "d7*2+P"
        "dd*1+P"
        "dd*8+P"
        "dd*8+P"
        "dd*25*1++P"
        "dd++2+P"
        "25*8*7+P"
        "25*dd*25*1++P"
        "dd*25*4++P"
        "dd*8+P"
        "d*P"
        "H"
    )
    return ''.join(seq)

def build_fib():
    seq = (
        "0p25*P"
        "1p25*P"
        "1p25*P"
        "2p25*P"
        "3p25*P"
        "5p25*P"
        "8p25*P"
        "25*3+p25*P"
        "25*25*1++p25*P"
        "25*dd++4+p25*P"
        "H"
    )
    return ''.join(seq)

if __name__ == '__main__':
    hello_seq = build_hello()
    hello_grid = place_on_grid(101, 100, hello_seq)
    with open('examples/hello_world.jump', 'w') as f:
        f.write(grid_to_str(hello_grid))
        f.write('\n')

    fib_seq = build_fib()
    fib_grid = place_on_grid(101, 100, fib_seq)
    with open('examples/fibonacci.jump', 'w') as f:
        f.write(grid_to_str(fib_grid))
        f.write('\n')

    print("Generated hello_world.jump and fibonacci.jump")
