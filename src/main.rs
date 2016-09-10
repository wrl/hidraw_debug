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

        let report_nr = buf[0];
        let report_bytes = &buf[1..nbytes];

        print!(" {: >3} bytes of {:02X}:", nbytes, report_nr);

        for b in report_bytes {
            print!(" {:02X}", b);
        }
        println!("");
    }
}

fn main() {
    let path = env::args().nth(1).expect("need device path");

    let fd = open(&*path, O_RDONLY, Mode::empty()).expect("couldn't open device");
    main_loop(fd)
}
