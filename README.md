# educlient-rs

[![CircleCI](https://dl.circleci.com/status-badge/img/gh/Anfeket/educlient-rs/tree/main.svg?style=svg)](https://dl.circleci.com/status-badge/redirect/gh/Anfeket/educlient-rs/tree/main)
![Lines of code](https://img.shields.io/tokei/lines/github/Anfeket/educlient-rs)

*very basic* Edupage API written in Rust

## Usage

```rust
use educlient::Educlient;

fn main() {
    let client = Educlient::new("domain")

    let grades = client.get_grades().1;
    println!("{}", grades);
}
```
*working example in src/main.rs*

## WIP
