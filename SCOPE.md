# Scope of Work — JUMP

## v1: Spec + Interpreter

**Goal**: A working knight-language that can run programs.

- [ ] Language specification document (JUMP-SPEC-v1.md)
  - Grid semantics, knight movement rules, priority order
  - Complete instruction set with edge cases
  - IO model (ASCII in/out)
- [ ] Grid parser: read `.jump` files into 2D array
- [ ] Knight VM in Rust:
  - [ ] IP state: (x, y, direction-history)
  - [ ] Stack: `Vec<i64>`
  - [ ] Execution loop: find valid moves, select by priority, execute cell
  - [ ] Basic ops: `+ - * /` (integer math), `:` dup, `\` swap, `$` drop
  - [ ] I/O: `.` number out, `,` ASCII out, `~` ASCII in
  - [ ] Control: `_` conditional jump priority flip, `@` halt
- [ ] CLI interpreter with `--file`, `--step` flags

**v1 Acceptance**: `hello.jump` outputs "Hello, World!"

## v2: CLI Debugger / Stepper

**Goal**: Understand what the knight is thinking.

- [ ] Step-through execution mode
- [ ] Stack inspection at each step
- [ ] Grid visualization in terminal (ANSI colors, knight position highlight)
- [ ] Breakpoints: halt when stack top equals a value or knight hits a coordinate
- [ ] Execution trace export (JSON/CSV)
- [ ] Reverse execution log (step back N moves)

## v3: Web Visualizer

**Goal**: The language comes alive in the browser.

- [ ] Rust core compiled to WASM (`wasm-bindgen`)
- [ ] HTML5 Canvas renderer:
  - [ ] Animated grid with knight movement
  - [ ] Animated stack visualization
  - [ ] Step / play / pause / back controls
  - [ ] Speed slider (1 step/sec to 60 steps/sec)
  - [ ] Color-coded instruction types
- [ ] Program gallery: load sample programs from dropdown
- [ ] Shareable URLs: base64-encoded program state

## v4: Compiler / Transpiler

**Goal**: JUMP is not just an interpreter language.

- [ ] Intermediate Representation (IR): linear stack-machine bytecode
- [ ] JUMP → IR compiler (static analysis of knight-traversable paths)
- [ ] IR → Brainfuck transpiler (proof of concept)
- [ ] IR optimizer: dead-code elimination, constant folding on reachable paths
- [ ] Optional: IR → native x86_64 (via cranelift or LLVM)

## Architecture

```
+----------+     +-----------+     +-------------+     +-----------+
| .jump    | --> |  Parser   | --> |  Knight VM  | --> |  Output   |
| source   |     | (grid)    |     | (stack + IP)|     | (stdout)  |
+----------+     +-----------+     +-------------+     +-----------+
                                         |
                                         v
                                  +-------------+
                                  |  WASM Bind  |
                                  |  (for web)  |
                                  +-------------+
                                         |
                                         v
                                  +-------------+
                                  | Web Renderer|
                                  | (Canvas)    |
                                  +-------------+
```

## Milestones

| Day | Deliverable | Acceptance Criteria |
|-----|-------------|-------------------|
| 1   | Specification | JUMP-SPEC-v1.md is complete and self-consistent |
| 2   | Interpreter | CLI can execute a `.jump` file and produce output |
| 3   | Hello World | `hello.jump` runs correctly; `fib.jump` produces sequence |
| 7   | Web Viz | Browser shows animated knight, stack, controls |
| 14  | Gallery | 5+ sample programs, shareable URLs, IR compiler sketch |

## Sample Programs Target (Day 14)

| Program | Description |
|---------|-------------|
| hello.jump | "Hello, World!" |
| fib.jump | First 20 Fibonacci numbers |
| prime.jump | Sieve of Eratosthenes |
| cat.jump | Identity program (echo input) |
| quine.jump | Self-replicating program |
| bf.jump | Brainfuck interpreter in JUMP |

## Non-Goals

- Standard library (JUMP is intentionally minimal)
- Multi-threading / parallel knight execution
- Floating-point arithmetic (integer only)
- Network I/O
- File system access beyond stdin/stdout
