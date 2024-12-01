# Benchmarking Journal

Here's a list of things I did and discovered!

## Parsing Optimizations
In both parts, the parsing function takes a significant portion of the run time... I focused on optimizing this first.

- (MINOR) Splitting directly on newline characters instead of using the .lines() method 
    - I reasoned that since the .lines() method automagically removes empty new
      lines at the end, performance might suffer. I could just handle this edge
      case myself while parsing.
- (MAJOR) Splitting the individual lines on `"   "` instead of whitespace
    - Not surprising that the compiler optimizes better when we can assure it
      of the general shape of the input. It doesn't need to waste time looking
      for more whitespace.
- (MINOR) Initialize vectors with capacity
    - When looking at the flamegraph, I noticed we were making calls to
      `RawVec::grow_one()`. Allocating the vectors with a starting capacity of
      1000 (exact size of the input) eliminates these calls!

- (DIDN'T WORK) Use a for loop when parsing
    - This was somewhat surprising, but it's in line with this 
      [section](https://doc.rust-lang.org/book/ch13-04-performance.html) of the
      Rust Book!


## Part 2 Optimizations
- (MAJOR) Using i32 instead of isize
    - I noticed this was improving part 2 by quite a bit, while not affecting
      part 1/parsing performance at all. I guess the hash function performs
      better on smaller values?
        - I wonder if I should try `i32`, though I'm worried about what the
          codspeed run will pass as input (we're boned if it's 32 bit ints). I
          should maybe open a github issue to ask?

