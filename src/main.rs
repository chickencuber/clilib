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
        file: String => [2],
        other: String,
    };
    rest => more: String;
}

cmd! {
    help: "usage: cmd [run|test]";
    .Data;
    :default_;
    run => "run",
    test => "test"|"alias",
}

fn default_(_: Data) {
    println!("this is the default one");
}

fn test(_: Data) {
    println!("this is a test");
}
fn run(d: Data) {
    println!("{:?}", d);
}
