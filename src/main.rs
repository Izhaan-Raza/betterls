use chrono::{DateTime, Utc};
use clap::Parser;
use owo_colors::OwoColorize;
use serde::Serialize;
use std::{
    fmt::format,
    fs,
    path::{Path, PathBuf},
};
use strum::Display;
use tabled::{
    Table, Tabled, grid::config::Border, settings::{
        Color, Style,
        object::{Column, Columns, Rows},
        style::BorderColor,
        themes::Colorization
    }
};

use std::os::unix::fs::PermissionsExt;

// Enum maxing 🚀🚀🚀
#[derive(Debug, Parser, Display, Serialize)]
enum EntryType {
    File,
    Dir,
}

// Struct to hold the metadata

#[derive(Debug, Tabled, Serialize)]
struct FileEntry {
    #[tabled(rename = "Perms")]
    permissions: String,
    #[tabled{rename="Name"}]
    name: String,
    #[tabled{rename="Type"}]
    e_type: EntryType,
    #[tabled{rename="Size"}]
    len_bytes: u64,
    modified: String,
}

// Made this struct for debuggin might not push to prod
#[derive(Debug, Parser)]
#[command(
    version,
    about,
    long_about = "Better LS command written in Rust cz Why not "
)]
struct Cli {
    path: Option<PathBuf>,
    #[arg(short, long)]
    json: bool,
}

fn main() {
    let cli = Cli::parse();

    let path = cli.path.unwrap_or(PathBuf::from("."));

    if let Ok(does_exist) = fs::exists(&path) {
        if does_exist {
            if cli.json {
                let get_files = get_files(&path);
                println!(
                    "{}",
                    serde_json::to_string(&get_files).unwrap_or("cannot parse".to_string())
                );
            } else {
                print_table(path);
            }
        } else {
            print!("{}", "Path does not exist.\n".red());
        }
    } else {
        println!("{} {}", "Error:".red(), "Unable to read the dir.\n".blue());
    }
}
// didnt need a seperate func for this but yeah...... it gets messy 
fn print_table(path: PathBuf) {
    let get_files = get_files(&path);
    let mut table = Table::new(get_files);

    table.with(Style::rounded());
    let color = Color::rgb_fg(100, 80, 160);
    table.with(BorderColor::filled(Color::rgb_fg(100, 80, 160)));
    table.modify(Rows::new(1..), BorderColor::new().top(color.clone()));
    table.modify(Columns::new(1..), BorderColor::new().left(color));
    
    table.modify(Columns::first(), Color::rgb_fg(105, 105, 105));
    table.modify(Columns::one(1), Color::FG_BRIGHT_CYAN);
    table.modify(Columns::one(2), Color::FG_BRIGHT_YELLOW);
    table.modify(Columns::one(4), Color::FG_BRIGHT_MAGENTA);
    table.modify(Rows::first(), Color::FG_BRIGHT_GREEN);
    
    println!("{}", table);
}

fn get_files(path: &Path) -> Vec<FileEntry> {
    let mut data = Vec::default();
    if let Ok(read_dir) = fs::read_dir(path) {
        for entry in read_dir {
            if let Ok(file) = entry {
                map_data(file, &mut data);
            }
        }
    }
    data
}

fn map_data(file: fs::DirEntry, data: &mut Vec<FileEntry>) {
    if let Ok(meta) = fs::metadata(&file.path()) {
        let perm_str = format_mode(meta.permissions().mode());
        data.push(FileEntry {
            permissions: perm_str,
            name: file
                .file_name()
                .into_string()
                .unwrap_or("Unknwo Name".into()),
            e_type: if meta.is_dir() {
                EntryType::Dir
            } else {
                EntryType::File
            },
            len_bytes: meta.len(),
            modified: if let Ok(modi) = meta.modified() {
                let date: DateTime<Utc> = modi.into();
                format!("{}", date.format("%a %b %e %Y"))
            } else {
                String::default()
            },
        });
    }
}

fn format_mode(mode: u32) -> String {
    let mut s = String::new();
    s.push_str(if mode & 0o400 != 0 { "r" } else { "-" });
    s.push_str(if mode & 0o200 != 0 { "w" } else { "-" });
    s.push_str(if mode & 0o100 != 0 { "x" } else { "-" });
    // Group
    s.push_str(if mode & 0o040 != 0 { "r" } else { "-" });
    s.push_str(if mode & 0o020 != 0 { "w" } else { "-" });
    s.push_str(if mode & 0o010 != 0 { "x" } else { "-" });
    // Others
    s.push_str(if mode & 0o004 != 0 { "r" } else { "-" });
    s.push_str(if mode & 0o002 != 0 { "w" } else { "-" });
    s.push_str(if mode & 0o001 != 0 { "x" } else { "-" });
    s
}
