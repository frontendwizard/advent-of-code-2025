# 🎄 Advent of Code 2025

**Learning Rust one puzzle at a time.**

A TypeScript developer's journey into Rust through [Advent of Code 2025](https://adventofcode.com/2025). Each day brings new challenges, compiler battles, and "aha!" moments.

## Progress

| Day | Puzzle | Stars | Learnings |
|-----|--------|-------|-----------|
| 01 | [Secret Entrance](day01/) | ⭐⭐ | Parsing, pattern matching, `rem_euclid`, enums |
| 02 | [Gift Shop](day02/) | ⭐⭐ | String slicing, iterators, basic threading |
| 03 | [Lobby](day03/) | ⭐⭐ | Greedy algorithms, `Option`, window iteration |
| 04 | [Printing Department](day04/) | ⭐⭐ | `Arc<T>`, threading strategies, grid traversal |

## Running Solutions

```bash
# Run a specific day
cargo run -p day01 -- day01/input.txt

# Run tests
cargo test -p day01

# Run with optimizations
cargo run -p day01 --release -- day01/input.txt
```

## Highlights

### Threading Adventures (Day 4)

Learned the hard way that parallelism granularity matters:

| Approach | Time |
|----------|------|
| Serial | 4.9ms |
| Thread per row | 2.9ms ✨ |
| Thread per cell | 2,637ms 💀 |

Spawning 10,000 threads for nanoseconds of work = bad idea.

### Rust vs TypeScript Mental Model

| TypeScript | Rust |
|------------|------|
| `npm` | `cargo` |
| `package.json` | `Cargo.toml` |
| `const` / `let` | `let` / `let mut` |
| `T \| undefined` | `Option<T>` |
| GC handles it | You handle it (with compiler help) |
| Runtime errors | Compiler yells at you |

## Structure

```
├── day01/          # Each day is a separate crate
│   ├── src/
│   │   └── main.rs
│   ├── Cargo.toml
│   └── README.md   # Puzzle description + notes
├── day02/
├── ...
└── Cargo.toml      # Workspace root
```

## The Journey

This repo documents my path from "what's a borrow checker?" to writing concurrent code with confidence.

---

*Built with ☕, 🦀, and occasional frustration at lifetime annotations.*
