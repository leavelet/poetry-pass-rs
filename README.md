# Poetry Pass

A memorable password generator using Chinese poetry and pinyin.

使用中国诗词和拼音生成易记密码的工具。

## Features

- **Multiple modes**: Full pinyin, initials, or original Chinese
- **Dual mode**: Different transformations for different parts
- **Configurable**: Custom separators, optional numbers
- **Poetry & words**: Mix classical poetry with common words

## Installation

```bash
cargo add poetry-pass
```

## Usage

```rust
use poetry_pass::{Generator, generate, generate_chinese};

// Default generation (full pinyin)
let password = generate();
// Example: "yingwu-guilaojianghubian-6694"

// Chinese original
let password = generate_chinese();
// Example: "鹦鹉-归老江湖边-5678"

// Initials only
let password = Generator::new().initials().generate();
// Example: "yw-gljhb-9012"

// Dual mode: front full pinyin, back initials
let password = Generator::new().front_full_back_init().generate();
// Example: "huaduo-hlzdc-1234"

// Custom configuration
let password = Generator::new()
    .poetry_only()
    .separator("_")
    .no_number()
    .generate();
// Example: "yingwu_guilaojianghubian"

// Generate with source for memorization
let (password, source) = Generator::new().generate_with_source();
println!("Password: {}", password);
println!("Source: {}", source);
```

## API

### Generator Methods

- `new()` - Create with default settings
- `initials()` - Use pinyin initials
- `full_pinyin()` - Use full pinyin (default)
- `chinese()` - Keep original Chinese
- `poetry_only()` - Use only poetry
- `words_only()` - Use only words
- `separator(sep)` - Set custom separator
- `no_number()` - Don't add random number
- `generate()` - Generate password
- `generate_with_source()` - Generate with source text

### Convenience Functions

- `generate()` - Quick generation with defaults
- `generate_chinese()` - Quick Chinese generation

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
