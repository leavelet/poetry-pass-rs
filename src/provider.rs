use rand::seq::IndexedRandom;
use rand::Rng;

#[derive(Clone, Debug)]
pub enum Source {
    Poetry,  // Poetry lines
    Words,   // Words list
    Mixed,   // Mixed source (70% words, 30% poetry)
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
        }
    }
    
    /// Get a random poetry line
    pub fn get_poetry(&self) -> &'static str {
        let mut rng = rand::rng();
        crate::POETRY.choose(&mut rng)
            .expect("No poetry data available")
    }
}