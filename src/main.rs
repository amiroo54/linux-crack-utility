use std::{collections::HashMap, env, fs::{self, File}, io::Write};
use byteorder::{ByteOrder, LittleEndian};
use md5::compute;
use colored::Colorize;
use std::process::Command;

pub mod onlinefix;
use onlinefix::*;

pub mod lutris;
use lutris::*;


fn main() {
    /*
    let arg = env::args().nth(1).unwrap();
    let bytes = get_shortcuts_vdf_string(arg);
    let mut file = check_shortcuts_vdf();
    file.write_all(&bytes).unwrap();
    */
    //println!("{}", decode_database());
    let args: Vec<String> = env::args().collect();
    match args.get(1).as_deref() {
        Some("help") | None => help(),
        Some("check") => check(),
        Some("lutris") => lutris(),
        Some("steam") => steam(),
        _ => println!("Invalid command. Use 'help' for available commands."),
    }
}

fn help()
{
    println!("{}", "Linux Crack Utility - LCU".blue().bold());
    println!("{}", "Usage:".green().bold());
    println!("{}", "    lcu [COMMAND] [OPTIONS]".bright_green());
    println!("{}", "Commands:".green().bold());
    let mut commands: HashMap<&str, &str> = HashMap::new();
    commands.insert("steam", "To add GoldTeam/OnlineFix games to steam");
    commands.insert("lutris", "To add an already installed game to lutris");
    for (command, description) in commands {
        println!("  {:<10} : {}", command.bright_green(), description.bright_black());
    }
    
}

fn check()
{
    let output = Command::new("wine").arg("--version").output();

    match output {
        Ok(output) => {
            if output.status.success() {
                println!("{} {}", "Wine".red(), "is installed".on_green())
            } else {
                println!("{} {}", "Wine".red(), "is not installed".on_red());
            }
        }
        Err(_) => println!("{} {}", "Wine".red(), "is not installed".on_red()),
    }
    let output = Command::new("lutris").arg("--version").output();

    match output {
        Ok(output) => {
            if output.status.success() {
                println!("{} {}", "Lutris".bright_green(), "is installed".on_green());
            } else {
                println!("{} {}", "Lutris".bright_green(), "is not installed".on_red());
            }
        }
        Err(_) => println!("{} {}", "Lutris".bright_green(), "is not installed".on_red()),
    }
}