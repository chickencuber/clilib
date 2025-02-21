use std::env::args;
use clilib::define; 


define! {
    Data;
    flags {
        t: bool = "w"|"h",   
        f: String = "f",
    };
    args {
        file: String,
    };
}

fn main() {
    println!("{:?}", Data::from(args().collect()))
}
