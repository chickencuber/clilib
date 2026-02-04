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
    run => "run",
    test => "test"|"alias",
}


fn test(_: Vec<String>) {
    println!("this is a test");
}
fn run(args: Vec<String>) {
    let d = Data::from(args.into_iter().skip(1).collect());
    println!("{:?}", d);
}
