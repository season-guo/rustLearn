use std::env;

mod open_file;
mod process;
mod ps_utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <name or pid of target>", args[0]);
        std::process::exit(1);
    }
    //#[allow(unused)] // TODO: delete this line for Milestone 1
    let query = &args[1];

    let target = ps_utils::get_target(&query).expect(&format!("could not find {}", query,));

    if let Some(ref ps) = target {
        ps.print();
        let children = ps_utils::get_child_processes(ps.pid).expect("could not get child processes");
        for child in children {
            child.print();
        }
    }else{
        println!("could not find {}", query);
        std::process::exit(1);
    }

    //jlet ps = target.unwrap();

    //let fds = ps.list_fds().expect("Expected list_fds to find file descriptors, but it returned None");
    //println!("File descriptors: {:?}", fds);
}

#[cfg(test)]
mod test {
    use std::process::{Child, Command};

    fn start_c_program(program: &str) -> Child {
        Command::new(program)
            .spawn()
            .expect(&format!("Could not find {}. Have you run make?", program))
    }

    #[test]
    fn test_exit_status_valid_target() {
        let mut subprocess = start_c_program("./multi_pipe_test");
        assert_eq!(
            Command::new("./target/debug/inspect-fds")
                .args(&[&subprocess.id().to_string()])
                .status()
                .expect("Could not find target/debug/inspect-fds. Is the binary compiled?")
                .code()
                .expect("Program was unexpectedly terminated by a signal"),
            0,
            "We expected the program to exit normally, but it didn't."
        );
        let _ = subprocess.kill();
    }

    #[test]
    fn test_exit_status_invalid_target() {
        assert_eq!(
            Command::new("./target/debug/inspect-fds")
                .args(&["./nonexistent"])
                .status()
                .expect("Could not find target/debug/inspect-fds. Is the binary compiled?")
                .code()
                .expect("Program was unexpectedly terminated by a signal"),
            1,
            "Program exited with unexpected return code. Make sure you handle the case where \
            ps_utils::get_target returns None and print an error message and return status \
            1."
        );
    }
}
