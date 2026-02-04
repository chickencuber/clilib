# CMDPARSING
a macro to do command line argument parsing  
```rust
use std::env::args;
use cmdparsing::define; 


define! {
    Data; //name of the struct
help: "usage: cmd [file(2)] [other]"; //the usage of the command(shows when using -help, or --help(reserved flag))
      flags {
t: bool = "w"|"h",  // a flag that uses a boolean
       f: String = "f", // a flag of type string
       o: String = "l" => [2], //a flag of type string with 2 entries
      };
      args {
file: String => [2], //a positional argument with 2 entries(String)
          other: String, //a positional argument with one entry(String)
      };
      rest => more: String; //a rest argument that takes strings(takes the last arguments unless its broken by a flag)
}

fn main() {
    let d = Data::from(args().skip(1).collect());
    println!("{:?}", d);
}

```

That example would generate this struct 

```rust
struct Data {
t: bool,
       f: Option<String>,
       o: Vec<String>,
       file: Vec<String>,
       other: String,
       more: Vec<String>,
}
```
## there's also command pattern
```rust
cmd! { // this generates a main function and runs it for you
help: "usage: cmd [run|test]";
      run => "run", //the ident is the function name and the string is the key for the command
          test => "test"|"alias", //there are also aliases
}

fn test(_: Vec<String>) {
    println!("this is a test");
}
fn run(args: Vec<String>) {
    let d = Data::from(args.into_iter().skip(1).collect());
    println!("{:?}", d);
}
```
but if you want more control
```rust
cmd! { //this generates the struct Main, which has a method named run to run the command
    Main;
    help: "usage: cmd [run|test]";
    run => "run", //the ident is the function name and the string is the key for the command
    test => "test"|"alias", //there are also aliases
}

fn main() {
    Main::run(std::env::args().skip(1).collect());
}

fn test(_: Vec<String>) {
    println!("this is a test");
}
fn run(args: Vec<String>) {
    let d = Data::from(args.into_iter().skip(1).collect());
    println!("{:?}", d);
}
```
