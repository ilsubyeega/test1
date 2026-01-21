use std::process::Command;

fn main() {
    Command::new("cat").arg("../../rootfile").spawn().unwrap();
}
