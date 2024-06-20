use std::{fs::read, io::{self, stdin, stdout, Read}};
use termion::{color::DetectColors, raw::{self, IntoRawMode}};

fn main() {
    let mut buf: [u8;1] = [b'0'];
    let terminal = stdout().into_raw_mode().unwrap();
    loop {
       
        stdin().read(&mut buf).unwrap();
        if char::is_control(char::from(buf[0])) {
            print!("Control char: {:?}", buf[0]);
        }
        print!("{:?} {:?}\r\n", buf[0], char::from(buf[0]));
        if buf[0] == b'q' {
            break;
        }
    }

}
