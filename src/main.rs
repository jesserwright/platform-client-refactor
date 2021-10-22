use std::{
    env,
    fs::File,
    fs::{self, read_to_string},
    io::Write,
    panic,
    path::{self, Path, PathBuf},
    slice::SliceIndex,
};

// conditional stepping via "stepping" buttons ("next"/"previous")
// will require around 5,000 lines of code instead 100,000 (95% less code)
// embeded system queries

/*
Goals
- Do code mods without touching the business logic
- Move to parcel
- Move everything that is JS to one file, then reduce from there
- Copy new files using include_str!()
- Remove all dependencies that are unused from package.json
- Transpile jsx and alias "React.createElement" as "e"
- Replace scss with tailwind
- Remove top level directories DONE
*/

// This could be a package called "flatten code"

fn main() -> std::io::Result<()> {
    let mut path_buff: PathBuf = env::args()
        .collect::<Vec<String>>()
        .get(1)
        .expect("Expected repository path as argument")
        .to_owned()
        .into();

    delete_dirs_and_files(&path_buff);
    create_files(&path_buff);

    let mut index_jsx: Vec<String> = Vec::new();

    loop {
        if path_buff.is_dir() {
            for f in fs::read_dir(&path_buff)? {
                let path = f?.path();
                if path.is_dir() {
                    path_buff.push(path);
                }
            }
            // iterate through entries, push path if directory
            // if there rae no entries, then pop path
            // path_buff.push(p);
        } else if path_buff.is_file() {
            let file = fs::read_to_string(&path_buff)?;
            //
            index_jsx.push(file);
        } else {
            path_buff.pop();
        }
        // read all files that end in .jsx or .js and concat to src/index.jsx
        // modify the source path as the directories are traversed
        // if it is a file, read the contents and append to src/index.jsx
        // if it is a directory, append the directory name to the src path and read entries of directory until
        // `read_dir` returns an error
        // anything that is .mock.js or .test.js or .stories.jsx remove
        // any time a file is appended to the main file, write the src path at the top for reference
        // all scss gets placed into a src/index.scss

        // stop when the root dir is empty and equal to the original; then create the final output file
        println!("{:?}", index_jsx);
    }

    Ok(())
}

// Delete files and directories
fn delete_dirs_and_files(root_path: &Path) -> std::io::Result<()> {
    let path_buff: PathBuf = root_path.into();
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
fn create_files(path: &Path) -> std::io::Result<()> {
    let path_buff: PathBuf = path.into();
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
