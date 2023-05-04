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
    let project_name: String;
    if path.is_some() {
        root = path.unwrap();
        std::fs::create_dir_all(root).expect("Failed to create new directory");
        project_name = root.to_owned();
    } else {
        root = "./";
        project_name = std::env::current_dir()
            .unwrap()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
    }

    let sdk_path = match std::env::var(SDK_ENV_VAR) {
        Ok(path) => path,
        Err(e) => {
            println!("{}: {}", SDK_ENV_VAR, e);
            std::process::exit(1);
        }
    };

    let re = Regex::new("PROJECT_NAME").unwrap();
    let cmake_template = re.replace_all(CMAKE_TEMPLATE, &project_name);
    let justfile_template = re.replace_all(JUSTFILE_TEMPLATE, &project_name);

    let re = Regex::new("SDK_PATH").unwrap();
    let cmake_template = re.replace_all(&cmake_template, sdk_path);

    let mut project_main =
        File::create(format!("{}/{}.c", root, project_name)).expect("Could not create new file");

    let mut cmake_list =
        File::create(format!("{}/CMakeLists.txt", root)).expect("Could not create CMakeLists.txt");

    let mut justfile =
        File::create(format!("{}/justfile", root)).expect("Could not create justfile");

    cmake_list
        .write_all(cmake_template.as_bytes())
        .expect("Failed to write CMakeLists.txt");

    project_main
        .write_all(MAIN_TEMPLATE.as_bytes())
        .expect("Failed to write project main file");

    justfile
        .write_all(justfile_template.as_bytes())
        .expect("Failed to write justfile");
}

const CMAKE_TEMPLATE: &'static str = r#"cmake_minimum_required(VERSION 3.13)

include(SDK_PATH/pico_sdk_init.cmake)

project(PROJECT_NAME)

pico_sdk_init()

add_executable(PROJECT_NAME
    PROJECT_NAME.c
)

target_link_libraries(PROJECT_NAME pico_stdlib)

pico_add_extra_outputs(PROJECT_NAME)"#;

const MAIN_TEMPLATE: &'static str = r#"#include "pico/stdlib.h"

int main() {
    
}"#;

const JUSTFILE_TEMPLATE: &'static str = r#"default:
    @just build
    @just load

build:
    @cmake .
    @make

load:
    @picotool load PROJECT_NAME.elf
    @picotool reboot"#;
