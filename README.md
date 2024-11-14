# uwurs
### UwUify your strings with uwurs! This Rust library transforms text into the playful "UwU" style by applying various character transformations, adding stutters, leetifying text, and more.

```toml
[dependencies]
uwurs = "0.3.1"
rand = "0.8.5"
```

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

API Reference
UwUifier Struct
Rust
pub struct UwUifier {
    pub emoticons: Vec<&'static str>,
    pub interjections: Vec<&'static str>,
    pub stutter_probability: f64,
    pub emoji_probability: f64,
    pub custom_map: HashMap<String, String>,
    pub emoji_map: HashMap<String, String>,
}
Methods
new() -> Self
Creates a new UwUifier instance with default settings.

uwuify(&self, input: &str) -> String
Transforms the input string into UwU style.

set_stutter_probability(&mut self, probability: f64)
Sets the probability for stuttering.

set_emoji_probability(&mut self, probability: f64)
Sets the probability for adding emojis.

add_emoticon(&mut self, emoticon: &'static str)
Adds a new emoticon to the list.

add_interjection(&mut self, interjection: &'static str)
Adds a new interjection.

add_custom_mapping(&mut self, from: &str, to: &str)
Adds a custom word mapping.

add_emoji_mapping(&mut self, word: &str, emoji: &str)
Associates a word with an emoji.

leetify(&self, input: &str) -> String
Converts the input string to leet speak.

reverse_text(&self, input: &str) -> String
Reverses the input string.

Transformation Functions
apply_character_transformations
Transforms specific characters to UwU style.

apply_stutter
Adds a stutter to a word based on probability.

leetify
Converts characters to their leet equivalents.

reverse_text
Reverses the input string.

Examples
Basic Uwuification
Rust
use uwurs::UwUifier;

fn main() {
    let uwuifier = UwUifier::new();
    let input = "I love programming!";
    let uwuified = uwuifier.uwuify(input);
    println!("{}", uwuified);
    // Possible Output: "I wove pwogwamming!"
}
Custom Mappings and Emojis
Rust
use uwurs::UwUifier;

fn main() {
    let mut uwuifier = UwUifier::new();
    uwuifier.add_custom_mapping("friend", "fwiend");
    uwuifier.add_emoji_mapping("happy", "😊");
    
    let input = "Hello, my friend! I am happy.";
    let uwuified = uwuifier.uwuify(input);
    println!("{}", uwuified);
    // Possible Output: "Hewwo, my fwiend! I am happy 😊."
}
Leetify and Reverse Text
Rust
use uwurs::UwUifier;

fn main() {
    let uwuifier = UwUifier::new();
    
    let leet = uwuifier.leetify("Leetify this text.");
    println!("{}", leet);
    // Output: "1337ify th15 73xt."
    
    let reversed = uwuifier.reverse_text("Reverse this.");
    println!("{}", reversed);
    // Output: ".siht esreveR"
}
License
This project is licensed under the MIT License.

Authors
Keiran - KeiranScript
Repository
GitHub Repository
Documentation
Homepage
Contact
For any inquiries or support, please contact keiran0@proton.me.

Contributing
Contributions are welcome! Please submit a pull request or open an issue on the GitHub Repository.

Acknowledgments
Thanks to the Rust community for their continuous support and contributions.
Versioning
This documentation corresponds to uwurs version 0.3.1.

Changelog
[0.3.1] - 2024-04-27
Initial release with core UwUification features.
Future Improvements
Add more customization options for transformations.
Improve performance for large text inputs.
Expand emoji and emoticon libraries.
Support
If you encounter any issues or have suggestions for improvements, feel free to open an issue on the GitHub repository.

Badges



Usage Examples in Code
Example 1: Basic Usage
Rust
use uwurs::UwUifier;

fn main() {
    let uwuifier = UwUifier::new();
    let text = "Hello, world!";
    let uwu_text = uwuifier.uwuify(text);
    println!("{}", uwu_text);
}
Example 2: Customizing UwUifier
Rust
use uwurs::UwUifier;

fn main() {
    let mut uwuifier = UwUifier::new();
    uwuifier.add_custom_mapping("Rust", "Ruwst");
    uwuifier.add_emoji_mapping("Rust", "🦀");
    uwuifier.set_stutter_probability(0.2);
    uwuifier.set_emoji_probability(0.3);

    let text = "I love Rust programming.";
    let uwu_text = uwuifier.uwuify(text);
    println!("{}", uwu_text);
}
Example 3: Using Transformation Functions
Rust
use uwurs::UwUifier;

fn main() {
    let uwuifier = UwUifier::new();

    let leet = uwuifier.leetify("Hello, World!");
    println!("{}", leet); // H3ll0, W0rld!

    let reversed = uwuifier.reverse_text("Hello, World!");
    println!("{}", reversed); // !dlroW ,olleH
}
Contribution Guidelines
If you'd like to contribute to uwurs, please follow these steps:

Fork the repository.
Create a new branch for your feature or bugfix.
Commit your changes with clear messages.
Submit a pull request detailing your changes.
Please ensure that your code follows the project's coding standards and includes appropriate tests.

Support and Feedback
Your feedback is valuable! If you encounter any issues or have suggestions for new features, please open an issue on GitHub.

Related Projects
rand - Random number generation used for stutter and emoji probabilities.
License
This project is licensed under the MIT License - see the LICENSE file for details.

Contact Information
Author: Keiran
Email: keiran0@proton.me
Website: https://keiran.cc/uwurs
Conclusion
uwurs provides a fun and customizable way to transform your text into the beloved UwU style. Whether you're looking to add flair to your messages or just have fun with text transformations, uwurs has you covered!

Quick Start
Install by adding to Cargo.toml.
Import and create an instance of UwUifier.
Transform your text using provided methods.
Customize settings and mappings as desired.
Enjoy UwUifying your text!

Screenshots
Coming soon!

License
This project is licensed under the MIT License - see the LICENSE file for details.

Acknowledgements
Inspired by the fun and creative nature of internet culture.
Future Work
Expand transformation rules for more varied UwU styles.
Integrate with other text processing libraries.
FAQs
Q: How can I customize the transformations?

A: Use methods like add_custom_mapping and add_emoji_mapping to define your own rules.

Q: Is uwurs thread-safe?

A: Yes, as long as you manage instances appropriately in concurrent contexts.

Q: Can I use uwurs in WebAssembly projects?

A: Yes, uwurs is compatible with WebAssembly.

Support
For support, please open an issue on the GitHub Repository.

Version History
0.3.1: Initial release with core UwUification features.
Contact
For more information, visit the homepage or contact keiran0@proton.me.

License
This project is licensed under the MIT License - see the LICENSE file for details.
