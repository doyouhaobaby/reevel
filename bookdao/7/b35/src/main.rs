use std::process::Command;

fn main() {
    Command::new("ls")
        .arg("-1")
        .arg("-a")
        .spawn()
        .expect("ls command failed to start");
}
