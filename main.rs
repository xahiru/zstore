#![allow(unused)]
use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;

// Ordering compares values
use std::cmp::Ordering;

fn say_hello(){
    println!("Hello");
}

fn main () {
    say_hello();
    //   let difficulty = 0x000fffffffffffffffffffffffffffff;
    //   let mut genesis_block = Block::new(0, now(), vec![0; 32],vec![Transaction {inputs: vec![ ],outputs: vec![ transaction::Output { to_addr: "Alice".to_owned(),value: 50,},transaction::Output {  to_addr: "Bob".to_owned(),value: 7,},],},], difficulty);
}