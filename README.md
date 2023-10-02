** Basic OS built using rust

- on apple silicon os runs by emulating an x86_64 machine
- qemu-system-x86_64 doesnt't work on m2 chip as instructed
- can run on apple silicon using UTM
- create a virtual machine with UEFI boot off
- load rust-bin as a disk image of interface IDE