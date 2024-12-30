# Containers from scratch

Exploring how Linux containers work at a low level.

## Usage

```sh
./run.sh
```

## Example

Setup:

```sh
$ ./run.sh
Running shell inside container...
Spawned child process with PID: 55365
[host] Setting up networking...
```

Commands inside container's shell:

```sh
/ # hostname
arnor
/ # hostname blah
/ # hostname
blah
/ # ps aux
PID   USER     TIME  COMMAND
    1 root      0:00 /bin/sh
    2 root      0:00 ps aux
/ # ip addr
1: lo: <LOOPBACK> mtu 65536 qdisc noop state DOWN qlen 1000
    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00
14: veth1@if15: <BROADCAST,MULTICAST,UP,LOWER_UP,M-DOWN> mtu 1500 qdisc noqueue state UP qlen 1000
    link/ether ae:75:24:ce:3a:76 brd ff:ff:ff:ff:ff:ff
    inet 192.168.43.2/24 scope global veth1
       valid_lft forever preferred_lft forever
    inet6 fe80::ac75:24ff:fece:3a76/64 scope link
       valid_lft forever preferred_lft forever
/ # ip route
default via 192.168.43.1 dev veth1
192.168.43.0/24 dev veth1 scope link  src 192.168.43.2
/ # ping 192.168.43.1
PING 192.168.43.1 (192.168.43.1): 56 data bytes
64 bytes from 192.168.43.1: seq=0 ttl=64 time=0.087 ms
^C
--- 192.168.43.1 ping statistics ---
1 packets transmitted, 1 packets received, 0% packet loss
round-trip min/avg/max = 0.087/0.087/0.087 ms
/ # ping 8.8.8.8
PING 8.8.8.8 (8.8.8.8): 56 data bytes
64 bytes from 8.8.8.8: seq=0 ttl=114 time=12.025 ms
^C
--- 8.8.8.8 ping statistics ---
1 packets transmitted, 1 packets received, 0% packet loss
round-trip min/avg/max = 12.025/12.025/12.025 ms
/ #
waitpid returned: 55365, status: 0

Commands on host:

```sh
$ hostname
arnor
```

## TODO

- Cgroup namespace
- User namespace
- Drop capabilities and run as non-root user
