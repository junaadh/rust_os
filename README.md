## Basic OS built using rust

- run in vm using qemu
- on apple silicon macs need UTM

### Commads

- install bootimage
```
  cargo install bootimage
```

- to compile and run target
```
  cargo run
```

- to run tests
```
  cargo test
```

- to manually build and spin up a vm
```
  cargo bootimage
```

  * non-apple silicon  
```
  qemu-system-x86_64 -drive format=raw,file=target/x86_64-rust_os/debug/bootimage-rust_os.bin
```
  
  * apple silicon
```
  qemu-system-x86_64 -L /Applications/UTM.app/Contents/Resources/qemu -drive format=raw,file=target/x86_64-rust_os/debug/bootimage-rust_os.bin
```
