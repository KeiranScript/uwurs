# uwurs
### UwUify your strings with uwurs! This Rust library transforms text into the playful "UwU" style by applying various character transformations, adding stutters, leetifying text, and more.

*This documentation corresponds to uwurs version 0.3.1*

```toml
[dependencies]
uwurs = "0.3.1"
rand = "0.8.5"
```

## Quick Start
- Install by adding to `Cargo.toml` or via `cargo add uwurs`
- Import and create an instance of UwUifier.
- Transform your text using provided methods.
- Customize settings and mappings as desired.
- Enjoy UwUifying your text!

## Usage
### Creating an Instance
To start using uwurs, create a new instance of UwUifier:

```Rust
use uwurs::UwUifier;

fn main() {
    let uwuifier = UwUifier::new();
    let input = "Hello, world!";
    let uwuified = uwuifier.uwuify(input);
    println!("{}", uwuified);
}
```

### Uwuify Text
Transform a string into UwU style:

```Rust
let input = "This is a test.";
let uwuified = uwuifier.uwuify(input);
println!("{}", uwuified);
```
### Leetify Text
Convert text to leet speak:

```Rust
let input = "Leetify this text.";
let leet = uwuifier.leetify(input);
println!("{}", leet);
```

### Reverse Text
Reverse the characters in a string:

```Rust
let input = "Reverse this.";
let reversed = uwuifier.reverse_text(input);
println!("{}", reversed);
```

## Configuration
### Setting Stutter Probability
Adjust the probability of adding stutters to words:

```rust
uwuifier.set_stutter_probability(0.2); // 20% chance
```

### Setting Emoji Probability
Adjust the probability of adding emojis:

```rust
uwuifier.set_emoji_probability(0.3); // 30% chance
```

### Adding Emoticons
Add custom emoticons to the UwUifier:

```rust
uwuifier.add_emoticon("(* ^ ω ^)");
```

### Adding Interjections
Add custom interjections:

```rust
uwuifier.add_interjection("nya");
```
### Adding Custom Mappings
Define custom word mappings:

```rust
uwuifier.add_custom_mapping("hello", "hewwo");
```

### Adding Emoji Mappings
Associate words with emojis:

```rust
uwuifier.add_emoji_mapping("happy", "😊");
```

## API Reference
### UwUifier Struct
```Rust
pub struct UwUifier {
    pub emoticons: Vec<&'static str>,
    pub interjections: Vec<&'static str>,
    pub stutter_probability: f64,
    pub emoji_probability: f64,
    pub custom_map: HashMap<String, String>,
    pub emoji_map: HashMap<String, String>,
}
```

### Methods

Create a new UwUifier instance with default settings
```rust
new() -> Self
```

Transform the input string into UwU style
```rust
uwuify(&self, input: &str) -> String
```

Set the probability for stuttering
```rust
set_stutter_probability(&mut self, probability: f64)
```

Set the probability for adding emojis
```rust
set_emoji_probability(&mut self, probability: f64)
```

Add a new emoticon to the list
```rust
add_emoticon(&mut self, emoticon: &'static str)
```

Add a new interjection
```rust
add_interjection(&mut self, interjection: &'static str)
```

Add a custom word mapping
```rust
add_custom_mapping(&mut self, from: &str, to: &str)
```

Associate a word with an emoji
```rust
add_emoji_mapping(&mut self, word: &str, emoji: &str)
```

Convert the input string to leet speak
```rust
leetify(&self, input: &str) -> String
```

Reverse the input string
```rust
reverse_text(&self, input: &str) -> String
```

### Transformation Functions
Transform specific characters to UwU style
```rust
apply_character_transformations
```

Add a stutter to a word based on probability
```rust
apply_stutter
```

Convert characters to their leet equivalents
```rust
leetify
```

Reverse the input string
```rust
reverse_text
```

## Examples
### Basic Uwuification

```Rust
use uwurs::UwUifier;

fn main() {
    let uwuifier = UwUifier::new();
    let input = "I love programming!";
    let uwuified = uwuifier.uwuify(input);
    println!("{}", uwuified);
}
```

### Custom Mappings and Emojis

```Rust
use uwurs::UwUifier;

fn main() {
    let mut uwuifier = UwUifier::new();
    uwuifier.add_custom_mapping("friend", "fwiend");
    uwuifier.add_emoji_mapping("happy", "😊");
    
    let input = "Hello, my friend! I am happy.";
    let uwuified = uwuifier.uwuify(input);
    println!("{}", uwuified);
}
```

### Leetify and Reverse Text

```Rust
use uwurs::UwUifier;

fn main() {
    let uwuifier = UwUifier::new();
    
    let leet = uwuifier.leetify("Leetify this text.");
    println!("{}", leet);
    
    let reversed = uwuifier.reverse_text("Reverse this.");
    println!("{}", reversed);
}
```

## Contributing
Contributions are welcome! Please submit a pull request or open an issue on the GitHub Repository.

If you'd like to contribute to uwurs, please follow these steps:

- Fork the repository.
- Create a new branch for your feature or bugfix.
- Commit your changes with clear messages.
- Submit a pull request detailing your changes.
- Please ensure that your code follows the project's coding standards and includes appropriate tests.

## License
### This project is licensed under the MIT License.
