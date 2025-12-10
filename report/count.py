import re
import sys
from pathlib import Path

def remove_commands(text: str) -> str:
    # Remove LaTeX commands like \command, \command{...}, \command[...]
    # 1. Remove commands with arguments: \cmd{...}, \cmd[...]
    text = re.sub(r'\\[a-zA-Z]+(\[[^\]]*\])?(\{[^}]*\})?', '', text)

    # 2. Remove standalone commands like \alpha, \LaTeX, etc.
    text = re.sub(r'\\[a-zA-Z]+', '', text)

    return text

def remove_math(text: str) -> str:
    # Remove $$...$$ display math
    text = re.sub(r'\$\$.*?\$\$', '', text, flags=re.DOTALL)

    # Remove $...$ inline math
    text = re.sub(r'\$.*?\$', '', text)

    # Remove \[...\] math
    text = re.sub(r'\\\[.*?\\\]', '', text, flags=re.DOTALL)

    # Remove equation-like environments
    math_envs = [
        "equation", "align", "align*", "gather", "gather*", 
        "multline", "multline*"
    ]
    for env in math_envs:
        text = re.sub(
            rf'\\begin\{{{env}\}}.*?\\end\{{{env}\}}',
            '',
            text,
            flags=re.DOTALL
        )
    return text

def strip_appendix(text: str) -> str:
    # Anything after \appendix or \begin{appendix} is removed
    appendix_markers = [
        r'\\appendix',
        r'\\begin\{appendix\}'
    ]
    for m in appendix_markers:
        match = re.search(m, text)
        if match:
            text = text[:match.start()]
            break
    return text

def count_characters(tex_path: str):
    text = Path(tex_path).read_text(encoding="utf-8")

    # Remove appendix sections
    text = strip_appendix(text)

    # Remove math content
    text = remove_math(text)

    # Remove latex commands
    text = remove_commands(text)

    # Remove comments
    text = re.sub(r'%.*', '', text)

    # Remove braces (optional, usually not part of "text")
    text = text.replace("{", "").replace("}", "")

    # Collapse multiple spaces/newlines
    clean_text = re.sub(r'\s+', ' ', text).strip()

    print("Clean text:\n")
    print(clean_text)
    print("\nCharacter count:", len(clean_text))

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python count_tex_chars.py file.tex")
        sys.exit(1)

    count_characters(sys.argv[1])
