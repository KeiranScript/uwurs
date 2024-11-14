# 🌸 uwurs 🌸

**uwurs** is a super cute Rust library that transforms your text into adorable "uwuified" versions! 🥰 It changes your sentences to mimic a playful and affectionate style, perfect for adding a touch of kawaii to your projects! ✨

## ✨ Features ✨

- **Cute Letter Substitutions**: Changes letters like `r` and `l` to `w` to create that classic uwu sound! 💖
- **Adorable Word Modifications**: Transforms words like `na` and `no` into `nya` and `nyo` for extra cuteness! 🥺
- **Punctuation Enhancements**: Adds sweet expressions like `uwu` and `owo` after punctuation marks! 😸
- **Random Emoticons & Interjections**: Sprinkles in kawaii emoticons and interjections to make your text super expressive! (・`ω´・)
- **Endearing Stutter Effect**: Randomly adds stutters to words to make them sound shy and precious! 🥰

## 🌷 Installation 🌷

Add **uwurs** to your `Cargo.toml` dependencies:

```toml
[dependencies]
uwurs = "0.1.0"
rand = "0.8"  # For extra randomness! ✨
```

Or install via Cargo:

```sh
cargo add uwurs
```

## 🍡 Usage 🍡

```rust
use uwurs::uwuify;

fn main() {
    let input = "Hello, world!";
    let uwuified = uwuify(input);
    println!("{}", uwuified);
}
```

Possible output:

```rust
Hewwo, wowwd! owo
```

## 🎀 Examples 🎀

```rust
let input = "Let's write some Rust code!";
let uwuified = uwuify(input);
println!("{}", uwuified);
```

Possible output:

```rust
Wet's wwite some Wust code! uwu (・`ω´・)
```
```rust
let input = "Programming is fun!";
let uwuified = uwuify(input);
println!("{}", uwuified);
```

Possible output:

```rust
Pwogwamming is fun! >w<
```

## 🌼 How It Works 🌼

- Letter Magic:
`r` or `l` -> `w`
`R` or `L` -> `W`

- Combos!!
`n` + `a`/`o` -> `ny` + `a`/`o` (e.g. `na` -> `nya`, `no` -> `nyo`)

- Punctuation:
Adds `owo`, `uwu` or cute emoticons after `?`/`!`/`.`

- Stutters:
Randomly adds stutters to words for extra adorableness! (e.g. `hello` -> `h-hewwo`)

## 🌸 Customisation 🌸

I plan to add more and more positional arguments to the `uwuify` function to allow for customisation of the uwuification process! 🌟

There are also more functions in the works to provide even more ways to uwuify your text! 🥰

## 💖 Testing 💖

We have a suite of tests to ensure that the uwuification process is as cute and accurate as possible! 🌈

e.g.

```rust
#[test]
fn test_uwuify_basic() {
    let input = "Hello, World!";
    let output = uwuify(input);
    assert!(output.contains("Hewwo") && output.contains("Wowwd"));
}
```

## 🌟 Contributing 🌟

Contributions are always welcome! Feel free to open issues or PRs if you have any suggestions or ideas! 🎉

Please check that your code passes the tests and that the documentation is up-to-date! 📚

## 📜 License 📜

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. 🌸
