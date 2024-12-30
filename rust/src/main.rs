use syscalls::{syscall, Sysno};

fn main() {
    // Spawn a new process using the clone3 syscall
    let flags = libc::CLONE_NEWUTS;
    let mut args = CloneArgs {
        flags: flags as u64,
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
    let args_ptr: usize = &mut args as *mut _ as usize;

    let result = unsafe { syscall!(Sysno::clone3, args_ptr, std::mem::size_of::<CloneArgs>()) };

    match result {
        Ok(0) => {
            // We are in the child process
            child_fn();
        }
        Ok(pid) => {
            // We are in the parent process
            println!("Spawned child process with PID: {}", pid);

            // Wait on the child process
            let mut status: i32 = 0;
            let options = 0;
            let result = unsafe { libc::waitpid(pid as i32, &mut status, options) };
            println!("waitpid returned: {:?}, status: {:?}", result, status);
        }
        Err(err) => {
            println!("Error spawning child process: {:?}", err);
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

const SHELL_PATH: &str = "/bin/sh\0";

fn child_fn() -> i32 {
    println!("Running shell inside container...");

    let result = unsafe {
        let path_ptr = SHELL_PATH.as_ptr();
        syscall!(Sysno::execve, path_ptr, 0, 0)
    };

    match result {
        Ok(_) => {
            println!("Shell exited successfully");
            0
        }
        Err(err) => {
            println!("Error running shell: {:?}", err);
            err.into_raw()
        }
    }
}
