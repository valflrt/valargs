A simple command-line argument helper for parsing positional arguments and options.

Example:

```rust
use valargs::Args;

fn main() {
    let args = Args::parse();

    if let Some(cat_name) = args.nth(1) {
        println!("the cat's name is {}", cat_name);
    }

    if args.has_option("orange") {
        println!("the cat is an orange cat")
    }

    if let Some(favorite_food) = args.option_value("favorite-food") {
        println!("the cat likes {} a lot", favorite_food)
    } else {
        println!("no information about the cat's favorite food...")
    }
}
```
