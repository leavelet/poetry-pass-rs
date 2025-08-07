use pinyin::ToPinyin as _;

#[derive(Clone, Debug)]
pub enum Mode {
    PinyinFull,  // Full Pinyin (default)
    PinyinInit,  // Initials
    Chinese,     // Original Chinese
}

impl Default for Mode {
    fn default() -> Self {
        Mode::PinyinFull  // Default to Full Pinyin
    }
}

#[derive(Clone, Debug)]
pub struct DualMode {
    pub front: Mode,
    pub back: Mode,
}

#[derive(Clone, Debug)]
pub enum TransformMode {
    Single(Mode), 
    Dual(DualMode),
}

impl Default for TransformMode {
    fn default() -> Self {
        TransformMode::Single(Mode::default())
    }
}

pub fn transform(text: &str, mode: &Mode) -> String {
    match mode {
        Mode::Chinese => text.to_string(),
        
        Mode::PinyinFull => {
            text.to_pinyin()
                .filter_map(|p| p.map(|py| py.plain().to_string()))
                .collect::<Vec<_>>()
                .join("")
        },
        
        Mode::PinyinInit => {
            text.to_pinyin()
                .filter_map(|p| p.map(|py| {
                    py.plain().chars().next().unwrap_or('?')
                }))
                .collect()
        },
    }
}

pub fn transform_dual(parts: &[&str], dual_mode: &DualMode) -> Vec<String> {
    parts.iter().enumerate().map(|(i, text)| {
        let mode = if i == 0 { &dual_mode.front } else { &dual_mode.back };
        transform(text, mode)
    }).collect()
}

pub fn transform_with_mode(parts: &[&str], transform_mode: &TransformMode) -> Vec<String> {
    match transform_mode {
        TransformMode::Single(mode) => {
            parts.iter().map(|text| transform(text, mode)).collect()
        },
        TransformMode::Dual(dual_mode) => {
            transform_dual(parts, dual_mode)
        }
    }
}
