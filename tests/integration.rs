#[cfg(test)]
mod tests {
    use poetry_pass::{Generator, generate, generate_chinese};
    
    #[test]
    fn test_default_generate() {
        let password = generate();
        assert!(!password.is_empty());
        assert!(password.contains('-'));
        // 默认应该是拼音，不应该包含中文字符
        assert!(!password.chars().any(|c| c as u32 > 127));
    }
    
    #[test]
    fn test_chinese_mode() {
        let password = generate_chinese();
        assert!(!password.is_empty());
        // 应该包含中文字符
        assert!(password.chars().any(|c| c as u32 > 127));
    }
    
    #[test]
    fn test_initials_mode() {
        let generator = Generator::new().initials();
        let password = generator.generate();
        assert!(!password.is_empty());
        // 首字母模式，长度应该较短
        let parts: Vec<&str> = password.split('-').collect();
        assert!(parts[0].len() < 10); // 首字母不会太长
    }
    
    #[test]
    fn test_multiple_generation() {
        let passwords = Generator::new().generate_multiple(10);
        assert_eq!(passwords.len(), 10);
        
        // 检查是否有重复（概率极小）
        use std::collections::HashSet;
        let unique: HashSet<_> = passwords.iter().collect();
        assert!(unique.len() > 7); // 至少70%应该是不同的
    }
    
    #[test]
    fn test_with_source() {
        let (password, source) = Generator::new().generate_with_source();
        assert!(!password.is_empty());
        assert!(!source.is_empty());
        // 来源应该是中文
        assert!(source.chars().any(|c| c as u32 > 127));
    }
}