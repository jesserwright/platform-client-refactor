use std::{
    env,
    ffi::OsString,
    fs,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

fn main() -> std::io::Result<()> {
    let repo_path: PathBuf = env::args()
        .collect::<Vec<String>>()
        .get(1)
        .expect("Path argument needed")
        .to_owned()
        .into();

    // .js files in source to delete *before* visiting
    fs::remove_dir_all(&repo_path.join("src/test-utils"))?;

    // Loading source into memory

    // package.json
    let vendor_packages = fs::read_to_string(repo_path.join("vendor-packages.js"))?;
    let mut package_json = fs::read_to_string(repo_path.join("package.json"))?;
    package_json.push_str(&vendor_packages);

    // js / jsx
    let mut js_buff = String::new();
    let patterns = vec![".js", ".jsx"];
    let patterns_not = vec![".test.js", ".stories.js", ".stories.jsx"];
    visit_dirs(
        &repo_path.join("src"),
        &mut js_buff,
        &patterns,
        &patterns_not,
    )?;

    let mut scss_buff = String::new();
    let patterns = vec![".scss"];
    visit_dirs(
        &repo_path.join("src"),
        &mut scss_buff,
        &patterns,
        &[].into(),
    )?;

    struct MyFile {
        name: OsString,
        body: String,
    }
    let mut translations_files: Vec<MyFile> = Vec::new();

    let translations_dir = repo_path.join("src/translations");
    for entry in fs::read_dir(&translations_dir)? {
        let entry = entry?;
        let path = entry.path();
        let translation_file = fs::read_to_string(path)?;
        translations_files.push(MyFile {
            body: translation_file,
            name: entry.file_name(),
        });
    }

    // Deletion
    delete_dirs_and_files(&repo_path)?;

    // Creation
    fs::create_dir_all(&translations_dir)?;
    for f in translations_files {
        let mut fd = File::create(translations_dir.join(f.name))?;
        fd.write_all(f.body.as_bytes())?;
    }

    create_files(&repo_path)?;

    let mut fd = File::create(repo_path.join("src/index.jsx"))?;
    fd.write_all(js_buff.as_bytes())?;
    let mut fd = File::create(repo_path.join("src/index.scss"))?;
    fd.write_all(scss_buff.as_bytes())?;

    let mut fd = File::create(repo_path.join("package.json"))?;
    fd.write_all(package_json.as_bytes())?;

    Ok(())
}

fn delete_dirs_and_files(root_path: &Path) -> std::io::Result<()> {
    let mut path_buff: PathBuf = root_path.into();
    for file_name in include_str!("to_delete.txt").lines() {
        path_buff.push(Path::new(file_name));
        if path_buff.exists() {
            let meta = fs::metadata(&path_buff)?;
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

fn create_files(path: &Path) -> std::io::Result<()> {
    let mut path_buff: PathBuf = path.into();
    for (file_name, file_str) in [
        (".gitignore", include_str!("./files_to_create/.gitignore")),
        (".prettierrc", include_str!("./files_to_create/.prettierrc")),
    ] {
        path_buff.push(&file_name);
        let mut fd = File::create(&path_buff)?;
        fd.write_all(file_str.as_bytes())?;
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
                            let components: Vec<_> =
                                path.components().map(|comp| comp.as_os_str()).collect();
                            let mut is_in_repo = false;
                            let mut new_path: Vec<_> = Vec::new();
                            for comp in components {
                                if comp == "platform-client" {
                                    is_in_repo = true
                                }
                                if is_in_repo {
                                    new_path.push(comp);
                                }
                            }

                            buff.push('\n');
                            buff.push_str("// "); // comment the path
                            buff.push_str(
                                new_path
                                    .iter()
                                    .collect::<PathBuf>()
                                    .as_path()
                                    .to_str()
                                    .unwrap(),
                            );
                            buff.push('\n');
                            buff.push_str(&s);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
