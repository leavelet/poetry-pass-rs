use crate::provider::{Provider, Source};
use crate::transform::{transform_with_mode, Mode, DualMode, TransformMode};
use rand::Rng;

pub struct Generator {
    source: Source,
    mode: TransformMode,
    separator: String,
    add_number: bool,
    random_capitalize: bool,
}

impl Generator {
    /// Create a new generator with default settings
    pub fn new() -> Self {
        Generator {
            source: Source::Mixed,
            mode: TransformMode::Single(Mode::PinyinFull), // Default to Full Pinyin
            separator: "-".to_string(),
            add_number: true,
            random_capitalize: false,
        }
    }
    
    /// Use initials (首字母)
    pub fn initials(mut self) -> Self {
        self.mode = TransformMode::Single(Mode::PinyinInit);
        self
    }

    /// Use full Pinyin
    pub fn full_pinyin(mut self) -> Self {
        self.mode = TransformMode::Single(Mode::PinyinFull);
        self
    }

    /// Use original Chinese (no transformation)
    pub fn chinese(mut self) -> Self {
        self.mode = TransformMode::Single(Mode::Chinese);
        self
    }
    
    /// Set dual mode with different modes for front and back parts
    pub fn dual_mode(mut self, front: Mode, back: Mode) -> Self {
        self.mode = TransformMode::Dual(DualMode { front, back });
        self
    }
    
    /// Convenience method: front full pinyin, back initials
    pub fn front_full_back_init(mut self) -> Self {
        self.mode = TransformMode::Dual(DualMode { 
            front: Mode::PinyinFull, 
            back: Mode::PinyinInit 
        });
        self
    }
    
    /// Convenience method: front initials, back full pinyin
    pub fn front_init_back_full(mut self) -> Self {
        self.mode = TransformMode::Dual(DualMode { 
            front: Mode::PinyinInit, 
            back: Mode::PinyinFull 
        });
        self
    }
    
    /// Use poetry source only
    pub fn poetry_only(mut self) -> Self {
        self.source = Source::Poetry;
        self
    }
    
    /// Use words source only
    pub fn words_only(mut self) -> Self {
        self.source = Source::Words;
        self
    }
    
    /// Use words-poetry combination (front: words, back: poetry)
    pub fn words_poetry(mut self) -> Self {
        self.source = Source::WordsPoetry;
        self
    }
    
    /// Use poetry-words combination (front: poetry, back: words)
    pub fn poetry_words(mut self) -> Self {
        self.source = Source::PoetryWords;
        self
    }
    
    /// Use words-words combination (front: words, back: words)
    pub fn words_words(mut self) -> Self {
        self.source = Source::WordsWords;
        self
    }
    
    /// Use poetry-poetry combination (front: poetry, back: poetry)
    pub fn poetry_poetry(mut self) -> Self {
        self.source = Source::PoetryPoetry;
        self
    }
    
    /// Set separator between parts
    pub fn separator(mut self, sep: &str) -> Self {
        self.separator = sep.to_string();
        self
    }
    
    /// Don't add a random number at the end
    pub fn no_number(mut self) -> Self {
        self.add_number = false;
        self
    }
    
    /// Enable random capitalization of at least one letter
    pub fn random_capitalize(mut self) -> Self {
        self.random_capitalize = true;
        self
    }
    
    /// Apply random capitalization to at least one letter in the password
    fn apply_random_capitalization(password: &str) -> String {
        let mut chars: Vec<char> = password.chars().collect();
        let mut rng = rand::rng();
        
        // Find all letter positions
        let letter_positions: Vec<usize> = chars
            .iter()
            .enumerate()
            .filter_map(|(i, &c)| if c.is_ascii_alphabetic() { Some(i) } else { None })
            .collect();
        
        if letter_positions.is_empty() {
            return password.to_string();
        }
        
        // Randomly choose 1-3 letters to capitalize (but at least 1)
        let num_to_capitalize = rng.random_range(1..=3.min(letter_positions.len()));
        
        // Randomly select positions to capitalize
        let mut selected_positions = Vec::new();
        for _ in 0..num_to_capitalize {
            loop {
                let pos = letter_positions[rng.random_range(0..letter_positions.len())];
                if !selected_positions.contains(&pos) {
                    selected_positions.push(pos);
                    break;
                }
            }
        }
        
        // Apply capitalization
        for &pos in &selected_positions {
            chars[pos] = chars[pos].to_ascii_uppercase();
        }
        
        chars.into_iter().collect()
    }
    
    /// Generate a random passphrase
    pub fn generate(&self) -> String {
        let provider = Provider::new(self.source.clone());
        let mut rng = rand::rng();
        
        let parts = match self.source {
            Source::Mixed => {
                vec![
                    provider.get(),
                    provider.get_poetry(),
                ]
            }
            Source::WordsPoetry | Source::PoetryWords | 
            Source::WordsWords | Source::PoetryPoetry => {
                vec![
                    provider.get_front(),
                    provider.get_back(),
                ]
            }
            _ => {
                vec![
                    provider.get(),
                    provider.get(),
                ]
            }
        };
        
        let transformed = transform_with_mode(&parts, &self.mode);
        
        let mut result = transformed.join(&self.separator);
        
        if self.add_number {
            let num: u32 = rng.random_range(1000..10000);  // 0.9: gen_range -> random_range, 4位数字
            result.push_str(&format!("{}{}", &self.separator, num));
        }
        
        if self.random_capitalize {
            result = Self::apply_random_capitalization(&result);
        }
        
        result
    }
    
    pub fn generate_multiple(&self, count: usize) -> Vec<String> {
        (0..count).map(|_| self.generate()).collect()
    }
    
    pub fn generate_with_source(&self) -> (String, String) {
        let provider = Provider::new(self.source.clone());
        
        let parts = match self.source {
            Source::Mixed => {
                vec![provider.get(), provider.get_poetry()]
            }
            Source::WordsPoetry | Source::PoetryWords | 
            Source::WordsWords | Source::PoetryPoetry => {
                vec![
                    provider.get_front(),
                    provider.get_back(),
                ]
            }
            _ => {
                vec![provider.get(), provider.get()]
            }
        };
        
        let source = parts.join(&self.separator);
        let transformed = transform_with_mode(&parts, &self.mode);
        
        let mut password = transformed.join(&self.separator);
        
        if self.add_number {
            let mut rng = rand::rng();
            let num: u32 = rng.random_range(1000..10000);
            password.push_str(&format!("{}{}", &self.separator, num));
        }
        
        if self.random_capitalize {
            password = Self::apply_random_capitalization(&password);
        }
        
        (password, source)
    }
}
