# JUMP

> *"The knight moves in L-shapes. So does your code."*

**JUMP** is an esoteric programming language where the instruction pointer moves across a 2D grid exactly like a chess knight. Stack-based. Memory-visual. Utterly impractical for real work and absolutely perfect for everything else.

## Paradigm

- **2D Grid**: Source code is laid out on a board. The IP (instruction pointer) is a knight.
- **Stack-Based**: All operations work on a single global stack.
- **Visual Memory**: Stack and grid state are rendered in real-time.
- **Deterministic**: No randomness. The knight's possible moves define the control flow.

## How It Works

The knight starts at the top-left `0,0`. From each cell, it can jump to up to 8 valid knight destinations. If multiple destinations are valid, execution follows a defined priority order (NNE, ENE, ESE, SSE, SSW, WSW, WNW, NNW). The instruction at the landed cell executes. Then the knight jumps again. Forever, until `@` (halt) is hit or the knight leaves the board.

## Instruction Set

| Symbol | Operation |
|--------|-----------|
| `0-9`  | Push digit to stack |
| `+`    | Pop two, push sum |
| `-`    | Pop two, push difference |
| `*`    | Pop two, push product |
| `/`    | Pop two, push integer division |
| `:`    | Duplicate top of stack |
| `\\`   | Swap top two stack items |
| `$`    | Pop and discard |
| `.`    | Pop and output as number |
| `,`    | Pop and output as ASCII character |
| `~`    | Read one character from input, push its ASCII |
| `!`    | Conditional: next jump skips cells where condition fails |
| `_`    | Pop; if zero, reverse jump priority order |
| `@`    | Halt |

## Sample Programs

### Hello World

```
72,101,108,108,111,32,87,111,114,108,100,33,,,,,,,,,,,,,,,,,,,,,,,
```

*(A 8×8 grid with ASCII values laid out in knight-traversable sequence.)*

### Fibonacci

```
0 1 : . + : . + : . + : . + : . + @
```

### Truth Machine

```
~ : . _ @
```

## Web Demo

A live visualizer runs in the browser via WASM + HTML5 Canvas:

```bash
cd jump-viz
wasm-pack build --target web
python3 -m http.server 8080
# open http://localhost:8080
```

Watch the knight hop, the stack grow, and your sanity slowly leave the board.

## Install (Interpreter)

```bash
cd jump-interpreter
cargo build --release
./target/release/jump --file hello.jump
```

## Debug Mode

```bash
./jump --file fib.jump --step
# Press Enter to advance one knight move
# Type 's' to show stack, 'q' to quit
```

---

*The board is infinite. The stack is unbounded. Your patience is neither.*

---

## ☕ Support the Developer

If this project saved you time, solved a problem, or just made your day a little more neon, you can fuel the next one:

[![Buy Me A Coffee](https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png)](https://buymeacoffee.com/synthalorian)
