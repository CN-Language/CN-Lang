#![feature(io_read_to_string)]
#![feature(with_options)]
#![feature(exclusive_range_pattern)]
#[macro_use]
extern crate nom;

mod cpp;
mod lexer;
mod parser;
fn main() {
    let emoji = '😊';
    let c = emoji as u64;
    println!("{:X}", c);
}
