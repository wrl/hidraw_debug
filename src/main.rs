use std::env;

extern crate nix;
use nix::fcntl::*;
use nix::unistd::*;
use nix::sys::stat::Mode;
use std::os::unix::io::RawFd;

fn main_loop(fd: RawFd) {
    let mut buf: [u8; 256] = [0; 256];

    loop {
        let nbytes = read(fd, &mut buf).unwrap();
        print!(" {: >3} bytes of {:02X}:", nbytes, buf[0]);

        for i in 1..nbytes {
            print!(" {:02X}", buf[i]);
        }
        println!("");
    }
}

fn main() {
    let path = env::args().nth(1).expect("need device path");

    let fd = open(&*path, O_RDONLY, Mode::empty()).expect("couldn't open device");
    main_loop(fd)
}
