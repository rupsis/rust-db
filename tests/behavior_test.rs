#[cfg(test)]
use os_pipe::pipe;
use std::io::prelude::*;
use std::process::{Command, Stdio};

// https://doc.rust-lang.org/rust-by-example/std_misc/process/pipe.html
// TODO write method to pipe in command to executable
// https://docs.rs/os_pipe/0.9.1/os_pipe/
pub fn run_script(){


    let mut child = Command::new("cargo");
    child.arg("run");

    let (mut reader, writer) = pipe().unwrap();
    let writer_clone = writer.try_clone().unwrap();
    child.stdout(writer);
    child.stderr(writer_clone);

    let mut handle = child.spawn().unwrap();

    // Doesn't seem to pipe the command to the child proces...

    child.arg(".exit");

    drop(child);

    let mut output = String::new();
    reader.read_to_string(&mut output).unwrap();
    handle.wait().unwrap();

    println!("{}", output);
}

mod tests {
    use super::*;


    #[test]
    fn it_works() {

    run_script();
        assert_eq!(2 + 2, 4);
    }
}