use std::env::args;
use cmdparsing::define; 


define! {
    Data;
    help: "usage: cmd [file(2)] [other]";
    flags {
        t: bool = "w"|"h",   
        f: String = "f",
        o: String = "l" => [2],
    };
    args {
        file: String => [2],
        other: String,
    };
    rest => more: String;
}

fn main() {
    let d = Data::from(args().collect());
    println!("{:?}", d);
}
