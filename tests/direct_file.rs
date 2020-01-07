use directio::DirectFileExt;
use std::fs::{self, File};
use std::io::{Read, Write};

#[test]
fn it_should_write_a_file() {
    let block = [b'a'; 4096];
    let file = File::direct_create("foo.txt");
    assert!(file.is_ok());

    let mut file = file.unwrap();
    assert!(file.write_all(&block).is_ok());

    fs::remove_file("foo.txt").unwrap();
}

#[test]
fn it_should_read_a_file() {
    let expected: [u8; 4096] = [b'a'; 4096];
    File::direct_create("bar.txt")
        .unwrap()
        .write_all(&expected)
        .unwrap();

    let file = File::direct_open("bar.txt");
    assert!(file.is_ok());

    let mut file = file.unwrap();
    let mut block = Vec::new();

    file.read_to_end(&mut block).unwrap();
    assert_eq!(expected[0..32], block[0..32]);

    fs::remove_file("bar.txt").unwrap();
}
