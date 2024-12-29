use std::mem;

use syscalls::{Sysno, syscall};

fn main() {
    // Spawn a new process using the clone3 syscall

    let mut args = CloneArgs {
        flags: 0,
        pidfd: 0,
        child_tid: 0,
        parent_tid: 0,
        exit_signal: 0,
        stack: 0,
        stack_size: 0,
        tls: 0,
        set_tid: 0,
        set_tid_size: 0,
        cgroup: 0,
    };
    let args_ptr: usize = &mut args as *mut _ as usize;
    let result = unsafe { syscall!(Sysno::clone3, args_ptr, mem::size_of::<CloneArgs>()) };
    println!("clone3 returned: {:?}", result);

    match result {
        Ok(pid) => {
            if pid == 0 {
                // We are in the child process
                println!("Inside the child process!");
                child_fn();
            } else {
                // We are in the parent process
                println!("Spawned child process with PID: {}", pid);
            }
        }
        Err(err) => {
            println!("Error spawning child process: {:?}", err);
        }
    }

    // Sleep for a bit
    std::thread::sleep(std::time::Duration::from_secs(3));
}

// const STACK_SIZE: usize = 1048576; // 1024 * 1024
// const CHILD_STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

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

fn child_fn() -> u64 {
    println!("Hello from the child process!");
    0
}
