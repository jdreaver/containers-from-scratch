# Containers from scratch

Exploring how Linux containers work at a low level.

## Usage

```sh
./run.sh
```

## TODO

- Start with just running `/bin/sh` in a new process created with `clone3` and a new UTS namespace. Example from man page <https://man7.org/linux/man-pages/man2/clone3.2.html#EXAMPLES>
- Use Alpine image <https://alpinelinux.org/downloads/> (see mini root filesystem) and a mount namespace
