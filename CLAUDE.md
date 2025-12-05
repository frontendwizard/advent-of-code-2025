# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

Advent of Code 2025 solutions in Rust. This is a **learning project** - the user is an experienced TypeScript developer learning Rust.

## Tutoring Approach

**Do not solve puzzles directly.** Instead:
- Guide toward the solution with hints and questions
- Teach Rust concepts as they become relevant (ownership, borrowing, iterators, pattern matching, etc.)
- Compare Rust idioms to TypeScript equivalents when helpful
- Let the user write the code - review and suggest improvements
- Explain compiler errors when they occur

## Commands

```bash
cargo run --bin dayXX        # Run a specific day
cargo test --bin dayXX       # Test a specific day
cargo build --release        # Optimized build
```

## Structure

```
src/bin/dayXX.rs            # Each day's solution
src/lib.rs                  # Shared utilities (if needed)
input/dayXX.txt             # Puzzle inputs (not committed)
```
