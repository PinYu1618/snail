#![no_std]
#![no_main]

use snail_user::{open, write, read, OpenFlags, close};

#[macro_use]
extern crate snail_user;

#[no_mangle]
fn main() -> i32 {
    let test_str = "Hello world";
    let file1 = "file1\0";
    let fd = open(file1, OpenFlags::CREATE | OpenFlags::WRONLY);
    assert!(fd > 0);
    write(fd as usize, test_str.as_bytes());
    close(fd as usize);

    let fd = open(file1, OpenFlags::RDONLY);
    assert!(fd > 0);
    let mut buf = [0_u8; 100];
    let read_len = read(fd as usize, &mut buf) as usize;
    close(fd as usize);

    assert_eq!(
        test_str,
        core::str::from_utf8(&buf[..read_len]).unwrap()
    );

    println!("file test (simple) ok.");
    0
}