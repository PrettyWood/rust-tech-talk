---
# try also 'default' to start simple
theme: seriph
# random image from a curated Unsplash collection by Anthony
# like them? see https://unsplash.com/collections/94734566/slidev
background: ./images/rust-language.jpg
# apply any windi css classes to the current slide
class: 'text-center'
# https://sli.dev/custom/highlighters.html
highlighter: shiki
# some information about the slides, markdown enabled
info: |
  ## Slidev Starter Template
  Presentation slides for developers.

  Learn more at [Sli.dev](https://sli.dev)
---

# Tech Talk - Rust

---

# What is Rust?

Rust is a multi-paradigm programming language designed for **performance** and **safety**, especially safe concurrency.

- created in 2010 by Graydon Hoare (Mozilla Research)
- v1 in 2015
- voted _most loved programming language_ since 2016

<br>
<br>
<br>

<v-clicks>

- compiled & statically strongly typed language
- **no garbage collector**
- **no manual memory management** (memory safety guaranty by using a **borrow checker**)
- **immutable** by default 🎉🎉🎉
- open source

</v-clicks>

<div class="rust-image" />

<style>
.rust-image {
  background: url("../images/rust.png") no-repeat;
  background-size: contain;
  position: absolute;
  width: 300px;
  height: 300px;
  top: 150px;
  right: 100px;
}
</style>

---

|     |     |     |
|:---:|:---:|:---:|
|     | **Safe (no SEGVs**) | **Direct Memory Control** |
| **C**| | ✅ |
| **C++**|  | ✅ |
| **Ruby**| ✅ | |
| **Python**| ✅ | |
| **Go**| ✅ | |
| **Rust**| ✅ | ✅ |

---

# What is Rust good at ?

- Efficiency (low memory, max CPU utilization)
- Embedding into other languages (such as Python 😉)
- low latency
- high security
- low-level targets (embedded, kernel, direct hardware access...)

---

# Everything included

---
layout: custom-image
image: ./images/everything.gif
size: contain
---

---
layout: image-right
image: ./images/cargo.png
---

# Everything included

- Build system

<v-clicks>

- Package system
- Documentation generator
- Code formatter
- Linter
- Test framework
- Toolchain installer
- Benchmarks

</v-clicks>

---

# Great resources

- [Official Rust book](https://doc.rust-lang.org/book/title-page.html)
- [Rust by example](https://doc.rust-lang.org/rust-by-example/)
- [Crate registry](https://crates.io/)
- [Rust in a nutshell](https://cheats.rs/)
- [Learn with Rustlings](https://github.com/rust-lang/rustlings)

---
layout: custom-image
image: ./images/curiosity.jpg
size: contain
---

---

# Expression-oriented language

<v-clicks>

```rust{all|2}
fn add(x: i32, y:i32) -> i32 {
    return x + y;
}
```

```rust{all|2}
fn add(x: i32, y:i32) -> i32 {
    x + y
}
```

```rust{2|all}
fn add(x: i32, y:i32) -> () {
    x + y;
}
```

```rust
let y = {
    let x_squared = x * x;
    let x_cube = x_squared * x;

    // This expression will be assigned to `y`
    x_cube + x_squared + x
};
```

</v-clicks>

---

# Pattern matching

```rust
fn my_function(x: i32, y:i32) -> i32 {
    if y == 0 {
      return -1;
    }

    let a = if x == 0 { -y } else { x + y };

    a + x + y
}
```

<v-clicks>

```rust
fn my_function(x: i32, y: i32) -> i32 {
    match (x, y) {
        (_, 0) => -1,
        (0, _) => 0,
        _ => 2 * (x + y),
    }
}
```

</v-clicks>

---

# Rust data structures

- `struct`: define a structure (AND)
- `enum`: define an enumeration (OR)
- `trait`: share common behaviour (implements some interface)

---

# Rust data structure: Result
<br>

[Result](https://doc.rust-lang.org/std/result/enum.Result.html)

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

<v-click>

[Example](https://doc.rust-lang.org/rust-by-example/error/result.html)

```rust{5|5-8|7|9|10|all}
use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let number_str = "10";
    let number = match number_str.parse::<i32>() {
        Ok(number)  => number,
        Err(e) => return Err(e),
    };
    println!("{}", number);
    Ok(())
}
```

</v-click>

---

# Rust data structure: Option
<br>

[Option](https://doc.rust-lang.org/std/option/enum.Option.html)

```rust
enum Option<T> {
    None,
    Some(T),
}
```

<v-click>

[Example](https://doc.rust-lang.org/rust-by-example/error/option_unwrap.html)

```rust
fn give_commoner(gift: Option<&str>) {
    // Specify a course of action for each case.
    match gift {
        Some("snake") => println!("Yuck! I'm putting this snake back in the forest."),
        Some(inner)   => println!("{}? How nice.", inner),
        None          => println!("No gift? Oh well."),
    }
}
```

</v-click>

---

# Traits

[Example](https://doc.rust-lang.org/rust-by-example/trait.html)

```rust
struct Sheep { naked: bool, name: &'static str }
impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }
}

trait Animal {
    fn noise(&self) -> &'static str;
    fn talk(&self) {
        println!("I say {}", self.noise());
    }
}

impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        if self.is_naked() { "baaaaah?" } else { "baaaaah!" }
    }
    fn talk(&self) {
        println!("{} pauses briefly...", self.noise());
    }
}
```

---

# Traits

```rust
// `Centimeters`, a tuple struct that can be compared
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

// `Inches`, a tuple struct that can be printed
#[derive(Debug)]
struct Inches(i32);
```

---

# Ownership

### Ownership rules

- Each value in Rust has a variable that's called its owner
- There can only be one owner at a time
- When the owner goes out of scope, the value is dropped

### Copy and Move
```rust
let x = 5;
let y = x;
```
A scalar value is _copied_ (stack)

```rust
let x = String::from("Hello");
let y = x;
```
Rust _copies_ the **pointer** (stack)
Rust moves the **ownership** of data from x to y

---
layout: center
---

# LIVE DEMO
