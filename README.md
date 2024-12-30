# Containers from scratch

Exploring how Linux containers work at a low level.

## Usage

```sh
./run.sh
```

## TODO

- Fix `execve` not working on `/bin/sh` (I think env is broken because I see "applet not found")
- Fix `/proc` mount breaking host `/proc` once container exits
  - See <https://stackoverflow.com/questions/39864352/mount-after-clone-with-clone-newns-set-effects-parent>
- Use Alpine image <https://alpinelinux.org/downloads/> (see mini root filesystem) and a mount namespace
- Network namespace
- Cgroup namespace
- User namespace
- Drop capabilities and run as non-root user
