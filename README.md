# educlient-rs

[![CircleCI](https://dl.circleci.com/status-badge/img/gh/Anfeket/educlient-rs/tree/main.svg?style=svg)](https://dl.circleci.com/status-badge/redirect/gh/Anfeket/educlient-rs/tree/main)
![Lines of code](https://img.shields.io/tokei/lines/github/Anfeket/educlient-rs)

[![CircleCI Insight Report](https://dl.circleci.com/insights-snapshot/gh/Anfeket/educlient-rs/main/rust-build-workflow/badge.svg?window=30d)](https://app.circleci.com/insights/github/Anfeket/educlient-rs/workflows/rust-build-workflow/overview?branch=main&reporting-window=last-30-days&insights-snapshot=true)

*very basic* Edupage API written in Rust

## Usage

```rust
use educlient::Educlient;

fn main() {
    let client = Educlient::new("username", "password", "domain");

    let grades = client.get_grades().unwrap();
    println!("{}", grades);
}
```

working example in src/main.rs

## WIP
