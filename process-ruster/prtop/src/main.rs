use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    println!("{:>5} {:<15} {:<12} {}", "PID", "Name", "State", "Command Line");
    println!("{}", "-".repeat(60));

    for entry in fs::read_dir("/proc").unwrap() {
        if let Ok(path) = entry.unwrap().path().canonicalize() {
            if let Some(pid) = path.file_name().and_then(|s| s.to_str()).and_then(|s| s.parse::<i32>().ok()) {
                let cmdline = fs::read_to_string(format!("/proc/{}/cmdline", pid)).unwrap_or_default();
                let (name, state) = BufReader::new(fs::File::open(format!("/proc/{}/status", pid)).unwrap())
                    .lines()
                    .filter_map(|line| line.ok())
                    .map(|line| line.splitn(2, ':').map(|s| s.trim().to_owned()).collect::<Vec<_>>())
                    .filter(|parts| parts.len() == 2)
                    .fold((String::new(), String::new()), |(name, state), parts| {
                        match parts[0].as_str() {
                            "Name" => (parts[1].to_owned(), state),
                            "State" => (name, parts[1].to_owned()),
                            _ => (name, state),
                        }
                    });

                println!("{:>5} {:<15} {:<12} {}", pid, name, state, cmdline.replace('\0', " "));
            }
        }
    }
}