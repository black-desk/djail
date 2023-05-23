# djail

## Background

Systemd encourage DE to put launched app into a systemd service or scope to make
XDG application manageable as any other systemd unit.[^1]

[^1]: https://systemd.io/DESKTOP_ENVIRONMENTS/

But the systemd user daemon only create cgroup owner by current user. Which
means a application can easily move itself out from the cgroup systemd put it in
by write to cgroup.proc file in the target cgroup.

## How this work

I wrote a program take in a command to execute, but before execute it, there is
some extra work to do:

1. unshare mount namespace;
2. mount /sys/fs/cgroup as slave;
3. remount /sys/fs/cgroup as read only filesystem.

Which make the application itself cannot change anything in cgroupfs, it cannot:

1. write to controller;
2. create new cgroup.

## Build

```bash
cargo build --release
sudo setcap cap_sys_admin+ep ./target/release/djail
```

## Usage

```bash
exec ./djail -- bash
```
