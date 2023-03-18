
use std::io::{self, Write};
use std::process::Command;

pub fn graph(id: i32){

    let gra: String = String::from(
"digraph P {
edge [dir=forward, arrowhead=none];
node [fontsize=11, fixedsize=true, height=1.5, width=1.5];\n\n");
    let mut file = std::fs::File::create("data.dot").expect("create failed");

    file.write_all(gra.as_bytes()).expect("write failed");

    file.write_all(crate::matrix::create_dot_string(id).as_bytes()).expect("write failed");

    file.write_all("\n}".as_bytes()).expect("write failed");
}

pub fn dot_to_svg() {
    let output = Command::new("dot")
        .args(["-Kfdp", "-n", "-Tsvg", "-o", "data.svg", "data.dot"])
        .output()
        .expect("Failed to execute command");
    io::stdout().write_all(&output.stdout).unwrap();
}