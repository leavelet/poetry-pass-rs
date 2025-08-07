# process_data.py
import json
import re

def extract_poetry_lines(input_file, output_file):
    """从诗词JSON中提取适合做密码的诗句"""
    with open(input_file, 'r', encoding='utf-8') as f:
        poems = json.load(f)
    
    lines = []
    for poem in poems:
        # 提取诗句（paragraphs字段）
        if 'paragraphs' in poem:
            for paragraph in poem['paragraphs']:
                # 按句号、逗号分割
                sentences = re.split('[，。！？；]', paragraph)
                for sentence in sentences:
                    # 清理空白和标点
                    sentence = sentence.strip()
                    # 长度在5-7个字的句子最适合
                    if 5 <= len(sentence) <= 7:
                        # 过滤掉包含生僻字的句子
                        if not contains_rare_chars(sentence):
                            lines.append(sentence)
    
    # 去重
    lines = list(set(lines))
    
    # 保存到文件
    with open(output_file, 'w', encoding='utf-8') as f:
        for line in lines:
            f.write(line + '\n')
    
    print(f"提取了 {len(lines)} 条诗句")

def extract_hsk_words(input_file, output_file, max_level=3):
    """从HSK词汇表中提取常用词语"""
    with open(input_file, 'r', encoding='utf-8') as f:
        words_data = json.load(f)
    
    words = []
    for entry in words_data:
        # 提取简体字
        word = entry.get('simplified', '')
        # 检查级别
        levels = entry.get('level', [])
        
        # 只要包含HSK 1-3级别的词
        for level in levels:
            if any(f'new-{i}' in level for i in range(1, max_level + 1)):
                if 2 <= len(word) <= 4:  # 2-4个字的词语
                    words.append(word)
                break
    
    # 去重
    words = list(set(words))
    
    # 保存
    with open(output_file, 'w', encoding='utf-8') as f:
        for word in words:
            f.write(word + '\n')
    
    print(f"提取了 {len(words)} 个词语")

def contains_rare_chars(text):
    """检查是否包含生僻字"""
    # 这里可以定义一个常用字集合
    # 简单起见，检查是否都是常用汉字
    for char in text:
        if '\u4e00' <= char <= '\u9fff':
            continue  # 是汉字
        else:
            return True  # 包含非汉字字符
    return False

if __name__ == '__main__':
    # 处理诗词
    extract_poetry_lines('data/raw/tang300.json', 'data/poetry.txt')
    
    # 处理词语
    extract_hsk_words('data/raw/hsk3.json', 'data/words.txt')
