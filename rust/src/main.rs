use nix::sched;

fn main() {
    // Spawn a new process using the clone3 syscall
    let flags = sched::CloneFlags::empty();
    let signal = Some(nix::sys::signal::Signal::SIGCHLD as i32);
    #[allow(static_mut_refs)]
    let result = unsafe { sched::clone(Box::new(child_fn), &mut CHILD_STACK, flags, signal) };

    println!("clone3 returned: {:?}", result);

    match result {
        Ok(pid) => {
            // We are in the parent process
            println!("Spawned child process with PID: {}", pid);

            // Wait on the child process
            nix::sys::wait::waitpid(pid, None).expect("waitpid failed");
        }
        Err(err) => {
            println!("Error spawning child process: {:?}", err);
        }
    }
}

const STACK_SIZE: usize = 1048576; // 1024 * 1024
static mut CHILD_STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

fn child_fn() -> isize {
    println!("Hello from the child process!");

    // Sleep for a bit
    std::thread::sleep(std::time::Duration::from_secs(3));

    0
}
