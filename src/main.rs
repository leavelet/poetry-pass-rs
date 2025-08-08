use poetry_pass::{Generator, generate};
use std::env;

struct Config {
    show_source: bool,
    count: Option<usize>,
    generator: Generator,
}

impl Config {
    fn new() -> Self {
        Config {
            show_source: false,
            count: None,
            generator: Generator::new(),
        }
    }
    
    fn parse_args(args: &[String]) -> Self {
        let mut config = Config::new();
        let mut i = 1;
        
        while i < args.len() {
            match args[i].as_str() {
                "-s" | "--source" => {
                    config.show_source = true;
                }
                "-n" | "--count" => {
                    if i + 1 < args.len() {
                        config.count = args[i + 1].parse().ok();
                        i += 1; // Skip the count value
                    }
                }
                "-i" | "--initials" => {
                    config.generator = config.generator.initials();
                }
                "-c" | "--chinese" => {
                    config.generator = config.generator.chinese();
                }
                "-d" | "--dual" => {
                    config.generator = config.generator.front_full_back_init();
                }
                "--dual-reverse" => {
                    config.generator = config.generator.front_init_back_full();
                }
                "-r" | "--random-caps" => {
                    config.generator = config.generator.random_capitalize();
                }
                "--word-poem" => {
                    config.generator = config.generator.words_poetry();
                }
                "--poem-word" => {
                    config.generator = config.generator.poetry_words();
                }
                "--word-word" => {
                    config.generator = config.generator.words_words();
                }
                "--poem-poem" => {
                    config.generator = config.generator.poetry_poetry();
                }
                _ => {} // Ignore unknown arguments
            }
            i += 1;
        }
        
        config
    }
}

fn print_help() {
    println!("诗词密码生成器");
    println!("用法: poetry-pass [选项]");
    println!("选项:");
    println!("  -i, --initials       使用拼音首字母");
    println!("  -c, --chinese        使用中文原文");
    println!("  -d, --dual           双模式：前全拼后首字母");
    println!("  --dual-reverse       双模式：前首字母后全拼");
    println!("  -r, --random-caps    随机大写至少一个字母");
    println!("  -n, --count N        生成N个密码");
    println!("  -s, --source         显示密码来源");
    println!("  --word-poem          词语-诗句组合");
    println!("  --poem-word          诗句-词语组合");
    println!("  --word-word          词语-词语组合");
    println!("  --poem-poem          诗句-诗句组合");
    println!("  -h, --help           显示帮助");
    println!();
    println!("示例:");
    println!("  poetry-pass -d       # 生成如 huaduo-hlzdc 格式");
    println!("  poetry-pass --dual-reverse  # 生成如 hd-huaduolizidanci 格式");
    println!("  poetry-pass --word-poem     # 生成如 huaduo-yuelangxingxi1234 格式");
    println!("  poetry-pass --poem-word     # 生成如 yuelangxingxi-huaduo1234 格式");
    println!("  poetry-pass --word-poem -s  # 显示词语-诗句密码及来源");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 && (args.contains(&"-h".to_string()) || args.contains(&"--help".to_string())) {
        print_help();
        return;
    }
    
    if args.len() > 1 {
        let config = Config::parse_args(&args);
        
        if let Some(count) = config.count {
            for password in config.generator.generate_multiple(count) {
                println!("{}", password);
            }
        } else if config.show_source {
            let (password, source) = config.generator.generate_with_source();
            println!("密码: {}", password);
            println!("来源: {}", source);
        } else {
            println!("{}", config.generator.generate());
        }
    } else {
        // 默认生成一个密码
        println!("{}", generate());
    }
}
