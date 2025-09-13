use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use rayon::prelude::*;
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // 帮助信息
    if args.len() < 2 || args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        println!("用法: {} <目录路径> [后缀1,后缀2,...]", args[0]);
        println!("示例: {} /Users/abc/tools property,ini", args[0]);
        return Ok(());
    }

    let root = Path::new(&args[1]);

    // 后缀列表，如果没有传，默认 property,ini
    let allowed: Vec<String> = if args.len() >= 3 {
        args[2].split(',').map(|s| s.to_lowercase()).collect()
    } else {
        vec!["property".to_string(), "ini".to_string()]
    };

    visit_dirs_parallel(root, &allowed)?;
    Ok(())
}

fn visit_dirs_parallel(root: &Path, allowed: &Vec<String>) -> io::Result<()> {
    let mut stack = vec![root.to_path_buf()];

    while let Some(path) = stack.pop() {
        if path.is_dir() {
            let entries: Vec<PathBuf> = match fs::read_dir(&path) {
                Ok(iter) => iter.filter_map(|e| e.ok()).map(|e| e.path()).collect(),
                Err(_) => continue,
            };

            entries.par_iter().for_each(|entry| {
                if entry.is_dir() {
                    let _ = visit_dirs_parallel(entry, allowed);
                } else {
                    let _ = process_file(entry, allowed);
                }
            });
        } else {
            let _ = process_file(&path, allowed);
        }
    }

    Ok(())
}

fn process_file(path: &PathBuf, allowed: &Vec<String>) -> io::Result<()> {
    let file_type = match path.extension() {
        Some(ext) => ext.to_string_lossy().to_lowercase(),
        None => "".into(),
    };

    if !allowed.contains(&file_type) {
        return Ok(());
    }

    let metadata = match fs::metadata(path) {
        Ok(m) => m,
        Err(_) => return Ok(()),
    };

    println!("文件: {:?}, 大小: {} 字节, 类型: {}", path, metadata.len(), file_type);
    Ok(())
}
