use poetry_pass::{Generator, generate, generate_chinese};

fn main() {
    println!("=== 诗词密码生成器 ===\n");
    
    // 1. 默认生成（拼音全拼）
    println!("默认生成（拼音全拼）:");
    for _ in 0..3 {
        println!("  {}", generate());
    }
    
    // 2. 拼音首字母
    println!("\n拼音首字母:");
    let generator = Generator::new().initials();
    for _ in 0..3 {
        println!("  {}", generator.generate());
    }
    
    // 3. 中文原文
    println!("\n中文原文:");
    for _ in 0..3 {
        println!("  {}", generate_chinese());
    }
    
    // 4. 显示来源（方便记忆）
    println!("\n生成并显示来源:");
    let generator = Generator::new();
    for _ in 0..3 {
        let (password, source) = generator.generate_with_source();
        println!("  密码: {}", password);
        println!("  来源: {}\n", source);
    }
    
    // 5. 自定义配置
    println!("自定义（纯诗句，首字母，无数字）:");
    let generator = Generator::new()
        .poetry_only()
        .initials()
        .no_number();
    println!("  {}", generator.generate());
}