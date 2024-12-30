fn main() {
    // Spawn a new process using the clone3 syscall
    let mut args = CloneArgs {
        flags: 0,
        pidfd: 0,
        child_tid: 0,
        parent_tid: 0,
        exit_signal: libc::SIGCHLD as u64,
        stack: 0,
        stack_size: 0,
        tls: 0,
        set_tid: 0,
        set_tid_size: 0,
        cgroup: 0,
    };

    let result = unsafe {
        libc::syscall(
            libc::SYS_clone3,
            &mut args as *mut _ as u64,
            std::mem::size_of::<CloneArgs>() as u64,
        )
    };

    println!("clone3 returned: {:?}", result);

    match result {
        -1 => {
            println!("Error spawning child process: {:?}", result);
        }
        0 => {
            // We are in the child process
            child_fn();
        }
        pid => {
            // We are in the parent process
            println!("Spawned child process with PID: {}", pid);

            // Wait on the child process
            let mut status: i32 = 0;
            let options = 0;
            let result = unsafe { libc::waitpid(pid as i32, &mut status, options) };
            println!("waitpid returned: {:?}, status: {:?}", result, status);
        }
    }
}

#[repr(C)]
struct CloneArgs {
    flags: u64,        /* Flags bit mask */
    pidfd: u64,        /* Where to store PID file descriptor (int *) */
    child_tid: u64,    /* Where to store child TID, in child's memory (pid_t *) */
    parent_tid: u64,   /* Where to store child TID, in parent's memory (pid_t *) */
    exit_signal: u64,  /* Signal to deliver to parent on child termination */
    stack: u64,        /* Pointer to lowest byte of stack */
    stack_size: u64,   /* Size of stack */
    tls: u64,          /* Location of new TLS */
    set_tid: u64,      /* Pointer to a pid_t array (since Linux 5.5) */
    set_tid_size: u64, /* Number of elements in set_tid (since Linux 5.5) */
    cgroup: u64,       /* File descriptor for target cgroup of child (since Linux 5.7) */
}

fn child_fn() -> isize {
    println!("Hello from the child process!");

    // Sleep for a bit
    std::thread::sleep(std::time::Duration::from_secs(3));

    0
}
