use rand::seq::IndexedRandom;
use rand::Rng;

#[derive(Clone, Debug)]
pub enum Source {
    Poetry,      // Poetry lines
    Words,       // Words list
    Mixed,       // Mixed source (70% words, 30% poetry)
    WordsPoetry, // Front: words, Back: poetry
    PoetryWords, // Front: poetry, Back: words
    WordsWords,  // Front: words, Back: words
    PoetryPoetry, // Front: poetry, Back: poetry
}

pub struct Provider {
    source: Source,
}

impl Provider {
    pub fn new(source: Source) -> Self {
        Provider { source }
    }
    
    pub fn get(&self) -> &'static str {
        let mut rng = rand::rng();
        match self.source {
            Source::Poetry => {
                crate::POETRY.choose(&mut rng)
                    .expect("No poetry data available")
            }
            Source::Words => {
                crate::WORDS.choose(&mut rng)
                    .expect("No words data available")
            }
            Source::Mixed => {
                if rng.random_ratio(7, 10) {
                    crate::WORDS.choose(&mut rng).unwrap()
                } else {
                    crate::POETRY.choose(&mut rng).unwrap()
                }
            }
            // For specific source combinations, use get_front() method
            Source::WordsPoetry | Source::PoetryWords | 
            Source::WordsWords | Source::PoetryPoetry => {
                self.get_front()
            }
        }
    }
    
    /// Get front part based on source type
    pub fn get_front(&self) -> &'static str {
        let mut rng = rand::rng();
        match self.source {
            Source::WordsPoetry | Source::WordsWords => {
                crate::WORDS.choose(&mut rng)
                    .expect("No words data available")
            }
            Source::PoetryWords | Source::PoetryPoetry => {
                crate::POETRY.choose(&mut rng)
                    .expect("No poetry data available")
            }
            _ => self.get() // Fallback to original logic
        }
    }
    
    /// Get back part based on source type
    pub fn get_back(&self) -> &'static str {
        let mut rng = rand::rng();
        match self.source {
            Source::WordsPoetry | Source::PoetryPoetry => {
                crate::POETRY.choose(&mut rng)
                    .expect("No poetry data available")
            }
            Source::PoetryWords | Source::WordsWords => {
                crate::WORDS.choose(&mut rng)
                    .expect("No words data available")
            }
            _ => self.get_poetry() // Fallback to poetry for Mixed mode
        }
    }
    
    /// Get a random poetry line
    pub fn get_poetry(&self) -> &'static str {
        let mut rng = rand::rng();
        crate::POETRY.choose(&mut rng)
            .expect("No poetry data available")
    }
}
