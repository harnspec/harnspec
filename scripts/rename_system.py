import os

replacements = {
    "LEANSPEC_": "HARNSPEC_",
    "leanspec.db": "harnspec.db"
}

def replace_in_files(directory):
    for root, dirs, files in os.walk(directory):
        if ".git" in dirs:
            dirs.remove(".git")
        if "node_modules" in dirs:
            dirs.remove("node_modules")
        if "target" in dirs:
            dirs.remove("target")
            
        for file in files:
            if file.endswith(('.rs', '.toml', '.md', '.json', '.js', '.ts', '.sh', '.yml', '.yaml', '.code-workspace', '.example')):
                file_path = os.path.join(root, file)
                try:
                    with open(file_path, 'r', encoding='utf-8') as f:
                        content = f.read()
                    
                    new_content = content
                    for old, new in replacements.items():
                        new_content = new_content.replace(old, new)
                    
                    if new_content != content:
                        with open(file_path, 'w', encoding='utf-8') as f:
                            f.write(new_content)
                        print(f"Updated: {file_path}")
                except Exception as e:
                    print(f"Error processing {file_path}: {e}")

if __name__ == "__main__":
    base_dir = "d:/AI/Antipback/harnspec/lean-spec"
    # Process all relevant directories
    dirs_to_process = ["rust", "packages", "scripts", ".github", ".husky", "bin", "docs", "docs-site", "schemas", "deploy"]
    for d in dirs_to_process:
        process_dir = os.path.join(base_dir, d)
        if os.path.exists(process_dir):
            replace_in_files(process_dir)
            
    # Process root files
    for entry in os.scandir(base_dir):
        if entry.is_file() and entry.name.endswith(('.md', '.json', '.yaml', '.yml', '.ts', '.js', '.code-workspace')):
            file_path = entry.path
            try:
                with open(file_path, 'r', encoding='utf-8') as f:
                    content = f.read()
                new_content = content
                for old, new in replacements.items():
                    new_content = new_content.replace(old, new)
                if new_content != content:
                    with open(file_path, 'w', encoding='utf-8') as f:
                        f.write(new_content)
                    print(f"Updated: {file_path}")
            except Exception as e:
                print(f"Error processing {file_path}: {e}")
