# Day 4: Printing Department

You ride the escalator down to the printing department. They're clearly getting ready for Christmas; they have lots of large rolls of paper everywhere, and there's even a massive printer in the corner (to handle the really big print jobs).

Decorating here will be easy: they can make their own decorations. What you really need is a way to get further into the North Pole base while the elevators are offline.

"Actually, maybe we can help with that," one of the Elves replies when you ask for help. "We're pretty sure there's a cafeteria on the other side of the back wall. If we could break through the wall, you'd be able to keep moving. It's too bad all of our forklifts are so busy moving those big rolls of paper around."

If you can optimize the work the forklifts are doing, maybe they would have time to spare to break through the wall.

The rolls of paper (`@`) are arranged on a large grid; the Elves even have a helpful diagram (your puzzle input) indicating where everything is located.

For example:

```
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
```

The forklifts can only access a roll of paper if there are **fewer than four** rolls of paper in the eight adjacent positions. If you can figure out which rolls of paper the forklifts can access, they'll spend less time looking and more time breaking down the wall to the cafeteria.

In this example, there are **13** rolls of paper that can be accessed by a forklift (marked with `x`):

```
..xx.xx@x.
x@@.@.@.@@
@@@@@.x.@@
@.@@@@..@.
x@.@@@@.@x
.@@@@@@@.@
.@.@.@.@@@
x.@@@.@@@@
.@@@@@@@@.
x.x.@@@.x.
```

Consider your complete diagram of the paper roll locations. **How many rolls of paper can be accessed by a forklift?**

---

## Learnings: Parallelism Granularity

We implemented three approaches and benchmarked them:

| Approach | Time | Speedup |
|----------|------|---------|
| Serial | 6.6ms | baseline |
| Thread per row | 3.0ms | **2x faster** |
| Thread per cell | 2,637ms | 400x slower! |

### Why thread-per-cell was so slow

Spawning a thread is expensive:
- Stack allocation (~2MB default per thread)
- OS kernel calls
- Context switching overhead

The actual work per cell (counting 8 neighbors) takes *nanoseconds*, but spawning a thread takes *microseconds to milliseconds*. For a 100x100 grid, that's 10,000 threads spawned for trivial work.

### Key takeaways

1. **Granularity matters**: Spawn fewer threads with more work each
2. **`Arc<T>`**: Use Atomic Reference Counting to share data across threads safely
3. **Thread pools** (like `rayon`) handle this automatically with work-stealing
4. **Don't over-parallelize**: Sometimes serial is faster for small workloads

### Code patterns learned

```rust
use std::sync::Arc;
use std::thread;

// Share data across threads with Arc
let grid = Arc::new(data);

// Clone Arc for each thread (cheap - just increments counter)
let grid_clone = Arc::clone(&grid);
thread::spawn(move || {
    // thread owns grid_clone, shares underlying data
});
```
