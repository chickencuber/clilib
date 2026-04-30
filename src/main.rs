use cmdparsing::{define, cmd}; 

define! {
    Data;
    help: "usage: cmd [file(2)] [other]";
    flags {
        t: bool = "w"|"h",   
        f: String = "f",
        o: String = "l" => [2],
    };
    args {
    };
}

type T = Data;

cmd! {
    help: "usage: cmd [run|test]";
    .Data;
    :default_;
    run => "run",
    test => "test"|"alias",
}

fn default_(_: T) {
    println!("this is the default one");
}

fn test(_: T) {
    println!("this is a test");
}
fn run(d: T) {
    println!("{:?}", d);
}
