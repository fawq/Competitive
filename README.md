# Tricks for competitive programming

## Read data

### Read line

Python:

 ```Python
... = input()
```

Rust:

```Rust
use std::io::{stdin, BufRead};
...
let mut stdin = stdin().lock();
let mut read_line = || {
  let mut line = String::new();
  stdin.read_line(&mut line).unwrap();
  line
};
```

### Single parameter string

Python:

```Python
word: str = input()
```

Rust:

```Rust
let word = read_line().trim();
```

### Single parameter e.g. int

Python:

```Python
number: int = int(input())
```

Rust:

```Rust
let number: i32 = read_line().parse().unwrap();
```

### Couple well known values e.g. int

Python:

```Python
x, y, z = map(int, input().split())
```

Rust:

```Rust
let (x, y, z) = {
  let line = read_line();
  let mut parts = line.split_ascii_whitespace();
  let x: i32 = parts.next().unwrap().parse().unwrap();
  let y: i32 = parts.next().unwrap().parse().unwrap();
  let z: i32 = parts.next().unwrap().parse().unwrap();
  (x, y, z)
};
```

### Vector of values e.g. int

Python:

```Python
numbers: list[int] = list(map(int, input().split()))
```

Rust:

```Rust
let numbers: Vec<i32> = read_line()
  .split_ascii_whitespace()
  .map(|s| s.parse().unwrap())
  .collect();
```

## Get operations on values

### Max, min

Python:

```Python
import sys
...
max_integer: int = sys.maxsize
min_integer: int = -sys.maxsize - 1

max_float: float = float("inf")
min_float: float = float("-inf")
```

Rust:

```Rust
let max_integer = i32::MAX;
let min_integer = i32::MIN;

let max_float = f32::MAX;
let min_float = f32::MIN;
```

## Colections

### Links

Python:
[collections](https://docs.python.org/3/library/collections.html)
Rust:
[std::collections](https://doc.rust-lang.org/std/collections/index.html)
