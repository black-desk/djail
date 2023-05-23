use std::os::unix::process::CommandExt;
extern crate libc;

// This program take in a command from command line arguments, unshare mount
// namespace, remount /sys/fs/cgroup as readonly then exec the command.
//
// This program only failed when we don't know what to execute. When unshare or
// mount failed this program execute the command anyway.

// https://stackoverflow.com/a/42773525
fn check_errno<T: Ord + Default>(num: T) -> std::io::Result<T> {
    if num < T::default() {
        return Err(std::io::Error::last_os_error());
    }
    Ok(num)
}

fn main() -> Result<(), std::io::Error> {
    let dash = std::env::args().nth(1).expect("no args given");
    if dash != "--" {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "no -- found",
        ));
    }

    let args: Vec<String> = std::env::args().skip(2).collect();

    let mut cmd = std::process::Command::new(&args[0]);
    cmd.args(&args[1..]);

    let _ = jail();

    Err(cmd.exec())
}

fn jail() -> Result<(), std::io::Error> {
    let cgroup = std::ffi::CString::new("/sys/fs/cgroup")?;

    unsafe {
        let _ = check_errno(libc::unshare(libc::CLONE_NEWNS))?;
        let _ = check_errno(libc::mount(
            std::ptr::null(),
            cgroup.as_ptr() as *const i8,
            std::ptr::null(),
            libc::MS_REC | libc::MS_SLAVE,
            std::ptr::null(),
        ))?;

        let _ = check_errno(libc::mount(
            std::ptr::null(),
            cgroup.as_ptr() as *const i8,
            std::ptr::null(),
            libc::MS_REMOUNT | libc::MS_BIND | libc::MS_RDONLY,
            std::ptr::null(),
        ))?;
    }

    Ok(())
}
