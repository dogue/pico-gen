use clap::{Parser, Subcommand};
use regex::Regex;
use std::{fs::File, io::Write};

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Create a new project in a new directory")]
    New { path: String },

    #[command(about = "Initialize a new project in the current directory")]
    Init,
}

const SDK_ENV_VAR: &'static str = "PICO_SDK_PATH";

fn main() {
    let options = Cli::parse();

    match options.command {
        Commands::New { path } => new_project(Some(&path)),
        Commands::Init => new_project(None),
    }
}

fn new_project(path: Option<&str>) {
    let root;
    if path.is_some() {
        root = path.unwrap();
        std::fs::create_dir_all(root).expect("Failed to create new directory");
    } else {
        root = "./";
    }

    let sdk_path = match std::env::var(SDK_ENV_VAR) {
        Ok(path) => path,
        Err(e) => {
            println!("{}: {}", SDK_ENV_VAR, e);
            std::process::exit(1);
        }
    };

    let template = std::fs::read_to_string("template/CMakeLists.txt")
        .expect("Could not read CMakeLists template file");

    let re = Regex::new("PROJECT_NAME").unwrap();
    let template = re.replace_all(&template, root);

    let re = Regex::new("SDK_PATH").unwrap();
    let template = re.replace_all(&template, sdk_path);

    let _project_main =
        File::create(format!("{}/{}.c", root, root)).expect("Could not create new file");

    let mut cmake_list =
        File::create(format!("{}/CMakeLists.txt", root)).expect("Could not create CMakeLists.txt");

    cmake_list
        .write_all(template.as_bytes())
        .expect("Failed to write CMakeLists.txt");
}
