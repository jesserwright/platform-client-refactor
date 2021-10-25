use std::{
    env, fs,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

fn main() -> std::io::Result<()> {
    let path_buff: PathBuf = env::args()
        .collect::<Vec<String>>()
        .get(1)
        .expect("Expected repository path as argument")
        .to_owned()
        .into();

    let mut index_buf = String::new();
    let patterns = vec![".js", ".jsx"];
    let patterns_not = vec![".test.js", ".stories.js", ".stories.jsx"]; // stories?
    visit_dirs(&path_buff, &mut index_buf, &patterns, &patterns_not)?;
    let mut fd = File::create("index.jsx")?;
    fd.write_all(index_buf.as_bytes())?;

    let mut index_buf = String::new();
    let patterns = vec![".scss"];
    visit_dirs(&path_buff, &mut index_buf, &patterns, &[].into())?;
    let mut fd = File::create("index.scss")?;
    fd.write_all(index_buf.as_bytes())?;

    // delete_dirs_and_files(&path_buff)?;
    // create_files(&path_buff)?;

    Ok(())
}

// Delete files and directories
fn delete_dirs_and_files(root_path: &Path) -> std::io::Result<()> {
    let mut path_buff: PathBuf = root_path.into();
    for file_name in include_str!("to_delete.txt").lines() {
        path_buff.push(Path::new(file_name));
        if path_buff.exists() {
            let meta = fs::metadata(&path_buff).expect("Failed to find meta");
            if meta.is_dir() {
                fs::remove_dir_all(&path_buff)?;
            }
            if meta.is_file() {
                fs::remove_file(&path_buff)?;
            }
        }
        path_buff.pop();
    }
    return Ok(());
}

struct NewFile<'a> {
    file_name: &'a str,
    file_str: &'a str,
}

// TODO: move this to one file, or inline it as a raw string.
fn create_files(path: &Path) -> std::io::Result<()> {
    let mut path_buff: PathBuf = path.into();
    for f in [
        NewFile {
            file_name: ".gitignore",
            file_str: include_str!("./files_to_create/.gitignore"),
        },
        NewFile {
            file_name: ".prettierrc",
            file_str: include_str!("./files_to_create/.prettierrc"),
        },
    ] {
        path_buff.push(&f.file_name);
        let mut fd = File::create(&path_buff).expect("Failed to create file");
        fd.write_all(f.file_str.as_bytes())
            .expect("Failed to write to file");
        path_buff.pop();
    }
    Ok(())
}

fn visit_dirs(
    dir: &Path,
    buff: &mut String,
    patterns: &Vec<&str>,
    patterns_not: &Vec<&str>,
) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, buff, &patterns, &patterns_not)?;
            } else {
                if let Some(file_name) = entry.file_name().to_str() {
                    // write file name
                    // translations
                    // scss
                    // imports and exports
                    // remove all tests
                    if let Some(_) = patterns_not
                        .iter()
                        .find(|&pat_not| -> bool { file_name.ends_with(pat_not) })
                    {
                        // skip the file
                        continue;
                    };
                    for pat in patterns {
                        if file_name.ends_with(pat) {
                            let s = fs::read_to_string(&path)?;
                            buff.push_str(&s);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
