use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use directio::DirectOpenOptionsExt;

#[test]
fn it_should_write_a_file() {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .direct()
        .open("foo.txt");

    assert!(file.is_ok());

    let block: [u8; 4096] = [b'a'; 4096];
    let mut file = file.unwrap();

    assert!(file.write_all(&block).is_ok());

    fs::remove_file("foo.txt").unwrap();
}

#[test]
fn it_should_read_a_file() {
    let expected: [u8; 4096] = [b'a'; 4096];
    File::create("bar.txt")
        .unwrap()
        .write_all(&expected)
        .unwrap();

    let file = OpenOptions::new()
        .read(true)
        .direct()
        .open("bar.txt");
    assert!(file.is_ok());

    let mut file = file.unwrap();
    let mut block = Vec::new();

    file.read_to_end(&mut block).unwrap();
    assert_eq!(expected[0..32], block[0..32]);

    fs::remove_file("bar.txt").unwrap();
}
