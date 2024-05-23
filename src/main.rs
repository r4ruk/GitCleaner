use std::io::stdin;
use std::process::{Command, Output};

/// Enum defining types of branches in list
/// used for info printing
#[derive(Debug)]
enum BranchType {
    Remotes,
    Locals,
    Stales
}


fn main() {
    let mut input = String::new();

    let _output = Command::new("git")
        .arg("fetch")
        .output()
        .expect("Could not fetch from origin, is it a repository and is Internet connection given?");

    let locals = retrieve_locals();
    print_infos(BranchType::Locals, &locals);

    // stdin().read_line(&mut input).unwrap();

    let remotes = retrieve_remotes();
    print_infos(BranchType::Remotes, &remotes);

    let stales = get_stale_branchnames(locals, remotes);
    print_infos(BranchType::Stales, &stales);

    cleanup_stales(stales);

    stdin().read_line(&mut input).unwrap();
}

/// Prints informations about branch list.
fn print_infos(branch_type: BranchType, branch_names: &Vec<String>) {
    println!("{:?}", branch_type);
    println!("{:?}", branch_names);
    println!();
}

/// cleans all stale branches from local
fn cleanup_stales(stales: Vec<String>) {
    let current_active = retrieve_actually_active();
    println!("Currently active");
    println!("{}", current_active);

    for branch in stales {
        if branch != current_active {
            let output = Command::new("git")
                .arg("branch")
                .arg("-d")
                .arg(branch)
                .output()
                .expect("cannot delete branch locally");
            if output.status.success() {
                let message = String::from_utf8_lossy(&output.stdout);
                println!("{}", message)
            }
        }
    }
}

/// retrieves locally active branch
fn retrieve_actually_active() -> String {
    let output = Command::new("git")
        .arg("branch")
        .arg("--show-current")
        .output()
        .expect("no active branch");
    let stdout = String::from_utf8_lossy(&output.stdout);
    return stdout.to_string();
}

/// Retrieves a list of all local branches available
fn retrieve_locals() -> Vec<String> {
    let output = Command::new("git")
        .arg("branch")
        .output()
        .expect("No local repository in the given path");
    // println!("{:?}", output);

    build_list(output)
}

/// Retrieves a list of all remote branches available
fn retrieve_remotes() -> Vec<String> {
    let output = Command::new("git")
        .arg("branch")
        .arg("-r")
        .output()
        .expect("git should output at least any branch or this is not a tracked repository");
    // println!("{:?}", output);

    build_list(output)
}

/// Builds list from command line response to the specific commands
fn build_list(output: Output) -> Vec<String> {
    let mut templist = Vec::new();
    if output.status.success() {
        // read output from command answer.
        let stdout = String::from_utf8_lossy(&output.stdout);

        // split at the newline character
        let splitted = stdout.split("\n");

        for i in splitted {
            // trim names from any empty spaces
            let trimmed = i.trim();

            if trimmed != ""{
                templist.push(trimmed.to_string());
            }
        }
    }
    return templist
}


/// Gets Branches which have no matchin remote branch
fn get_stale_branchnames(local_branches: Vec<String>, remote_branches: Vec<String>) -> Vec<String> {

    let mut stales:Vec<String> = Vec::new();
    for mut branch in local_branches {

        // in case the branch is active it has a * and whitespace at the start. replace in any case
        branch = branch.replace("* ", "");

        // build remotename which is starting with origin/
        // in normal cases. its possible that a branch has another names remote but this
        // has to be defined explicitly and can therefore also be removed explicitly.
        let remotename = format!("origin/{}", branch);

        // ingore main and master branch in any case and push branches which have no matching remote
        if remote_branches.contains(&remotename) == false
            && branch != "master".to_string()
            && branch != "main"{
            stales.push(branch);
        }
    }
    return stales
}