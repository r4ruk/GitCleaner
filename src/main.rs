use std::io::stdin;
use std::process::Command;

fn main() {
    // println!("Hello, world!");

    let output = Command::new("dir")
        .output()
        .expect("asdf");
    println!("{:?}", output);

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let output = Command::new("git")
        .arg("branch")
        .arg("-r")
        .output()
        .expect("git should output at least any branch or this is not a tracked repository");
    println!("{:?}", output);

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let splitted = stdout.split("\n");
        for i in splitted {
            println!("{}", i);
        }
    }

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
}
