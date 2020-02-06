# eload

Load environment variables into your struct members directly.

```rust
use eload::load;
use serde::{Serialize, Deserialize};

// Step 1. Derive `Serialize` and `Deserialize`
#[derive(Serialize, Deserialize)]
struct A {
    a: u32,
    b: u32,
    c: u32,
}

// Step 2. Call eload::load
//
// Environment variables are loaded as follows:
// * `APP_A` -> A::a
// * `APP_B` -> A::b
let a = A { ... };
let a = load("app", &a).unwrap();
```

## Loading containers

Use YAML format to describe containers: `[1, 2, 3]` for vector, `{a: 1, b: 2}` for map.
