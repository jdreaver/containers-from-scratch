use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;

use clap::Parser;
use syscalls::{syscall, Sysno};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Location of the root filesystem to mount (host location; it will always mount to / in the container)
    #[arg(short, long, value_name = "FILE")]
    mount_root: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    // Spawn a new process using the clone3 syscall
    let flags = libc::CLONE_NEWUTS | libc::CLONE_NEWPID | libc::CLONE_NEWNS; // | libc::CLONE_NEWNET;
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
            child_fn(cli.mount_root);
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

fn child_fn(mount_root: PathBuf) -> i32 {
    println!("Running shell inside container...");

    // Mount the root filesystem
    let result = pivot_root_chroot(mount_root);
    if result != 0 {
        return result;
    }

    // Mount /proc, which is not shared with the host
    let src = c"proc".as_ptr();
    let target = c"/proc".as_ptr();
    let fstype = c"proc".as_ptr();
    let flags = 0;
    let data = std::ptr::null();
    let result = unsafe { libc::mount(src, target, fstype, flags, data) };
    if result < 0 {
        println!(
            "Error mounting /proc: {:?}",
            std::io::Error::last_os_error()
        );
        return result;
    }

    // Exec the shell
    let result = unsafe {
        let path_ptr = c"/bin/sh".as_ptr();
        let args = [
            path_ptr, // argv[0] is the program name
            std::ptr::null(),
        ];
        let env = [
            c"PATH=/bin:/sbin:/usr/bin:/usr/sbin".as_ptr(),
            std::ptr::null(),
        ];
        syscall!(Sysno::execve, path_ptr, args.as_ptr(), env.as_ptr())
    };

    if let Err(err) = result {
        println!("Error running shell: {:?}", err);
        return err.into_raw();
    }
    0
}

/// We can't use chroot to sandbox the filesystem. The man page specifically says:
///
/// > This call changes an ingredient in the pathname resolution process and
/// > does nothing else. In particular, it is not intended to be used for any kind
/// > of security purpose, neither to fully sandbox a process nor to restrict
/// > filesystem system calls.
///
/// Therefore, we use the pivot_root syscall (and some ceremony around it) to
/// change the container's root directory.
fn pivot_root_chroot(new_root: PathBuf) -> i32 {
    // Ensure current root filesystem doesn't have shared propagation
    let result = unsafe {
        libc::mount(
            std::ptr::null(),
            c"/".as_ptr(),
            std::ptr::null(),
            libc::MS_REC | libc::MS_PRIVATE,
            std::ptr::null(),
        )
    };
    if result < 0 {
        println!(
            "Error setting root filesystem to private
                {:?}",
            std::io::Error::last_os_error()
        );
        return result;
    }

    // Bind mount the root filesystem
    let new_root_ptr = new_root.as_os_str().as_bytes().as_ptr() as *const i8;
    let fstype = c"none".as_ptr();
    let flags = libc::MS_BIND | libc::MS_REC;
    let data = std::ptr::null();
    let result = unsafe { libc::mount(new_root_ptr, new_root_ptr, fstype, flags, data) };
    if result < 0 {
        println!(
            "Error bind mounting root filesystem {:?}: {:?}",
            new_root,
            std::io::Error::last_os_error()
        );
        return result;
    }

    // Create a directory for the old root inside the new root (for pivot_root)
    let old_root_path = std::path::Path::new(&new_root).join(".old_root");
    let old_root_path = old_root_path
        .to_str()
        .expect("failed to create .old_root path");
    std::fs::create_dir_all(old_root_path).expect("failed to create .old_root");

    // Change working directory to the new root
    std::env::set_current_dir(&new_root).expect("failed to change working directory");

    // Call pivot_root to switch to the new root
    let old_root_cstr =
        CString::new(old_root_path).expect("failed to convert .old_root path to CString");

    let result = unsafe {
        let put_old = old_root_cstr.as_ptr();
        syscall!(Sysno::pivot_root, new_root_ptr, put_old)
    };
    if let Err(err) = result {
        println!("Error calling pivot_root: {:?}", err);
        return err.into_raw();
    }

    // Change working directory to / in the new root
    std::env::set_current_dir("/").expect("failed to change working directory");

    // Unmount the old root and remove the directory
    let result = unsafe {
        let target = c".old_root".as_ptr();
        let flags = libc::MNT_DETACH;
        syscall!(Sysno::umount2, target, flags)
    };
    if let Err(err) = result {
        println!("Error unmounting old root: {:?}", err);
        return err.into_raw();
    }

    std::fs::remove_dir(".old_root").expect("failed to remove .old_root");
    0
}
