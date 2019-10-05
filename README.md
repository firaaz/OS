# To build the program
- cargo install cargo-xbuild (just once to install the crate)
- cargo xbuild
- cargo install bootimage --version "^0.7.7"

# to run the program 
- cargo bootimage
- qemu-system-x86_64 -drive format=raw,file=target/x86_64/debug/bootimage-os.bin (to run it in a virtualized env)
*the bootimage can be found at ./target/x86_64/debug/bootimage-os.bin (from project root)*

## *run all of this at the project root*
