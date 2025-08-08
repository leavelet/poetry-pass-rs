# Poetry Pass

A memorable password generator using Chinese poetry and pinyin.

使用中国诗词和拼音生成易记密码的工具。

## Features

- **Multiple modes**: Full pinyin, initials, or original Chinese
- **Dual mode**: Different transformations for different parts
- **Source specification**: Choose specific combinations (word-poem, poem-word, etc.)
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
// Example: "huaduo-hlzdc-1234" (花朵-红楼梦里的春天 -> huaduo-hlzdc)

// Dual mode: front initials, back full pinyin  
let password = Generator::new().front_init_back_full().generate();
// Example: "hd-honglouzichuntiande-5678" (花朵-红楼梦里的春天 -> hd-honglouzichuntiande)

// Custom configuration
let password = Generator::new()
    .poetry_only()
    .separator("_")
    .no_number()
    .random_capitalize()
    .generate();
// Example: "yingWu_guiLaojianghubIan"

// Generate with source for memorization
let (password, source) = Generator::new().generate_with_source();
println!("Password: {}", password);
println!("Source: {}", source);

// Specify source combinations
let password = Generator::new().words_poetry().generate();
// Example: "huaduo-yuelangxingxi-1234" (word-poem)
```

## API

### Generator Methods

- `new()` - Create with default settings
- `initials()` - Use pinyin initials
- `full_pinyin()` - Use full pinyin (default)
- `chinese()` - Keep original Chinese
- `poetry_only()` - Use only poetry
- `words_only()` - Use only words
- `words_poetry()` - Front: words, back: poetry
- `poetry_words()` - Front: poetry, back: words
- `words_words()` - Front: words, back: words
- `poetry_poetry()` - Front: poetry, back: poetry
- `dual_mode(front, back)` - Set different transformation modes for front and back parts
- `front_full_back_init()` - Front part uses full pinyin, back part uses initials
- `front_init_back_full()` - Front part uses initials, back part uses full pinyin
- `separator(sep)` - Set custom separator
- `no_number()` - Don't add random number
- `random_capitalize()` - Enable random capitalization
- `generate()` - Generate password
- `generate_with_source()` - Generate with source text
- `generate_multiple(count)` - Generate multiple passwords

### Convenience Functions

- `generate()` - Quick generation with defaults
- `generate_chinese()` - Quick Chinese generation


## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
