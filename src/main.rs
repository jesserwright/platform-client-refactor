use std::{env, fs, fs::File, io::Write};

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

// Take a look here: https://doc.rust-lang.org/std/fs/fn.read_dir.html

fn main() {
    let mut args = env::args().collect::<Vec<String>>();
    let repo_directory: &mut String = &mut args[1];
    repo_directory.push_str("/");
    let dirs_to_delete = [
        ".githooks",
        ".github",
        ".husky",
        ".loki",
        ".storybook",
        "codebuild",
        "config",
        "cypress",
        "dependency-build",
        "docs",
    ];
    for d in dirs_to_delete {
        let mut path = repo_directory.to_owned();
        path.push_str(d);
        fs::remove_dir_all(path).unwrap();
    }

    let files_to_delete = [
        ".npmrc",
        ".babelrc",
        ".dockerignore",
        ".eslintignore",
        ".eslintrc.js",
        ".stylelintignore",
        ".stylelintrc",
        "amplify.yml",
        "cypress.json",
        "initialize.sh",
        "jsconfig.json",
        "package-lock.json",
        "README.md",
        "server.amplify.js",
        "server.dev.js",
        "server.js",
        "server.prodmode.js",
        "vendor-packages.js",
        "yarn-error.log",
        ".prettierrc",
        ".gitignore",
    ];
    for f in files_to_delete {
        let mut path = repo_directory.to_owned();
        path.push_str(f);
        fs::remove_file(path).unwrap();
    }
    struct NewFile<'a> {
        file_name: &'static str,
        file_string: &'a str,
    }

    for f in [
        NewFile {
            file_name: ".gitignore",
            file_string: include_str!("./files/.gitignore"),
        },
        NewFile {
            file_name: ".prettierrc",
            file_string: include_str!("./files/.prettierrc"),
        },
    ] {
        let mut path = repo_directory.to_owned();
        path.push_str(f.file_name);
        let fd_result = File::create(path);
        if let Ok(mut fd) = fd_result {
            fd.write_all(f.file_string.as_bytes()).unwrap();
        }
    }

    /*
    cd src
    while [directories] {
        - open directory
        - any contents with .js or .jsx ending is read and appended to the index.jsx file
        - any file with .scss place into yet another file
        - delete file when done
        - if it ends in .mock.js or .test.js, then remove it without adding it to the main file
    }
    */
    loop {
        break;
    }
}
