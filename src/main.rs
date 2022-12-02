mod day1;
mod day2;
#[allow(unused)]
mod reader;

#[derive(Debug)]
pub enum Error {
    InputError,
}

pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    println!("Hello, world!");
}
