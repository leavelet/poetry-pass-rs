use crate::provider::{Provider, Source};
use crate::transform::{transform_with_mode, Mode, DualMode, TransformMode};
use rand::Rng;

pub struct Generator {
    source: Source,
    mode: TransformMode,
    separator: String,
    add_number: bool,
}

impl Generator {
    /// Create a new generator with default settings
    pub fn new() -> Self {
        Generator {
            source: Source::Mixed,
            mode: TransformMode::Single(Mode::PinyinFull), // Default to Full Pinyin
            separator: "-".to_string(),
            add_number: true,
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
        
        (password, source)
    }
}
