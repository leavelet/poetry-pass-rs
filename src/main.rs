use poetry_pass::{Generator, generate};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        match args[1].as_str() {
            "-h" | "--help" => {
                println!("诗词密码生成器");
                println!("用法: poetry-pass [选项]");
                println!("选项:");
                println!("  -i, --initials       使用拼音首字母");
                println!("  -c, --chinese        使用中文原文");
                println!("  -d, --dual           双模式：前全拼后首字母");
                println!("  --dual-reverse       双模式：前首字母后全拼");
                println!("  -n, --count N        生成N个密码");
                println!("  -s, --source         显示密码来源");
                println!("  -h, --help           显示帮助");
                println!();
                println!("示例:");
                println!("  poetry-pass -d       # 生成如 huaduo-hlzdc 格式");
                println!("  poetry-pass --dual-reverse  # 生成如 hd-huaduolizidanci 格式");
            }
            "-i" | "--initials" => {
                println!("{}", Generator::new().initials().generate());
            }
            "-c" | "--chinese" => {
                println!("{}", Generator::new().chinese().generate());
            }
            "-d" | "--dual" => {
                println!("{}", Generator::new().front_full_back_init().generate());
            }
            "--dual-reverse" => {
                println!("{}", Generator::new().front_init_back_full().generate());
            }
            "-s" | "--source" => {
                let (password, source) = Generator::new().generate_with_source();
                println!("密码: {}", password);
                println!("来源: {}", source);
            }
            "-n" | "--count" => {
                let count = args.get(2)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(5);
                for password in Generator::new().generate_multiple(count) {
                    println!("{}", password);
                }
            }
            _ => {
                println!("{}", generate());
            }
        }
    } else {
        // 默认生成一个密码
        println!("{}", generate());
    }
}
