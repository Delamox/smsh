use std::fs;
use std::io::{self, Write};
const FILE_NOT_FOUND_ERR: &str = "No such file or directory";
const PROCESS_SPAWN_ERR: &str = "Failed to spawn subprocess";
const STDOUT_ERR: &str = "Failed to read stdout";
const PATH_ERR: &str = "Failed to read HOME environment variable";

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
    let Ok(path_env) = std::env::var("PATH") else {
        println!("{}", PATH_ERR);
        return;
    };
    let paths: Vec<&str> = path_env.split(':').collect();
    for path in paths {
        if fs::metadata(format!("{}/{}", path, cmd)).is_ok() {
            if istype {
                println!("{} is {}/{}", cmd, path, cmd)
            } else {
                let exec = format!("{path}/{cmd}");
                run_program(&exec, &split[1..])
            }
            return;
        }
    }
    match istype {
        true => println!("{} not found", cmd),
        false => println!("{}: command not found", cmd),
    }
}

fn run_program(program: &str, args: &[&str]) {
    let Ok(process) = std::process::Command::new(program).args(args).spawn() else {
        println!("{}", PROCESS_SPAWN_ERR);
        return;
    };
    let Ok(stdout) = process.wait_with_output() else {
        println!("{}", STDOUT_ERR);
        return;
    };
    let Ok(stdout) = String::from_utf8(stdout.stdout) else {
        println!("{}", STDOUT_ERR);
        return;
    };
    print!("{}", stdout);
}

fn change_directory(dir: &str) {
    let realdir = match dir.starts_with('~') {
        true => str::replace(dir, "~", std::env::var("HOME").unwrap().as_str()),
        false => dir.to_string(),
    };
    let Ok(canonicalized) = fs::canonicalize(realdir) else {
        println!("{}: {}", dir, FILE_NOT_FOUND_ERR);
        return;
    };
    let path: String = canonicalized.display().to_string();
    match fs::metadata(&path).is_ok() {
        true => std::env::set_current_dir(&path).expect("error"),
        false => println!("{}: {}", &path, FILE_NOT_FOUND_ERR),
    }
}

fn main() {
    loop {
        let wd: String = std::env::current_dir().unwrap().display().to_string();
        let splitwd: &str = wd.split('/').next_back().unwrap();
        print!("[{}]$ ", splitwd);
        io::stdout().flush().unwrap();
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let split: Vec<&str> = input.trim().split(' ').collect();
        match split[..] {
            ["echo", ..] => {
                if split.len() > 1 {
                    println!("{}", split[1..].join(" "))
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
            ["pwd", ..] => println!("{}", std::env::current_dir().unwrap().display()),
            ["type", ..] => find_command(split, true),
            _ => find_command(split, false),
        }
    }
}
