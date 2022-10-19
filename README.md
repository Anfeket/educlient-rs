# educlient-rs
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
*working example in src/example.rs*