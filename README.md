# Containers from scratch

Exploring how Linux containers work at a low level.

## Usage

```sh
./run.sh
```

## TODO

- Fix `/proc` mount breaking host `/proc` once container exits
- Use Alpine image <https://alpinelinux.org/downloads/> (see mini root filesystem) and a mount namespace
- Network namespace
- Cgroup namespace
- User namespace
- Drop capabilities and run as non-root user
