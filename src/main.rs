mod day1;
mod day2;
mod day3;
#[allow(unused)]
mod reader;

#[derive(Debug)]
pub enum Error {
    InputError,
    ValidationError(&'static str),
}

pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    println!("Hello, world!");
}
