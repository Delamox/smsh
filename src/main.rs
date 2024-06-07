#[allow(unused_imports)]
use std::fs;
use std::io::{self, Write};

fn find_command(split: Vec<&str>, istype: bool) {
    let cmd: &str = if istype { split[1] } else { split[0] };
    if istype {
        match cmd {
            "echo" | "exit" | "type" | "pwd" | "cd" => {
                println!("{} is a shell builtin", cmd);
                return;
            }
            _ => (),
        }
    }
    let path_env = std::env::var("PATH").unwrap();
    let paths: Vec<&str> = path_env.split(':').collect();
    for path in &paths {
        if fs::metadata(format!("{}/{}", path, cmd)).is_ok() {
            match istype {
                true => {
                    println!("{} is {}/{}", cmd, path, cmd);
                }
                false => {
                    let executable = format!("{path}/{cmd}");
                    run_program(&executable, &split[1..])
                }
            }
            return;
        }
    }
    if istype {
        println!("{} not found", cmd)
    } else {
        println!("{}: command not found", cmd)
    }
}

fn run_program(program: &str, args: &[&str]) {
    let process = std::process::Command::new(program)
        .args(args)
        .spawn()
        .unwrap();
    let stdout = String::from_utf8(process.wait_with_output().unwrap().stdout).unwrap();
    print!("{}", stdout);
}

fn change_directory(dir: &str) {
    let realdir = if dir.chars().next().unwrap() == '~' {
        let changed = str::replace(dir, "~", std::env::var("HOME").unwrap().as_str());
        changed
    } else {
        dir.to_string()
    };
    let path: String = match fs::canonicalize(realdir) {
        Ok(ok) => ok.display().to_string(),
        Err(_error) => {
            println!("{}: No such file or directory", dir);
            return;
        }
    };
    if fs::metadata(&path).is_ok() {
        std::env::set_current_dir(&path).expect("error");
    } else {
        println!("{}: No such file or directory", &path)
    }
}

fn main() {
    loop {
        let wd: String = std::env::current_dir().unwrap().display().to_string();
        print!("[{}]$ ", wd);
        io::stdout().flush().unwrap();
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let split: Vec<&str> = input.trim().split(' ').collect();
        match split[..] {
            ["echo", ..] => {
                if split.len() > 1 {
                    println!("{}", split[1..].join(" "));
                }
            }
            ["exit", ..] => break,
            ["cd", ..] => {
                if split.len() > 1 {
                    if split.len() > 2 {
                        println!("cd: Too many arguments");
                        return;
                    }
                    change_directory(split[1].trim())
                }
            }
            ["pwd", ..] => println!("{}", std::env::current_dir().unwrap().display().to_string()),
            ["type", ..] => find_command(split, true),
            _ => find_command(split, false),
        }
    }
}
