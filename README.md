# CLILIB
a macro to do command line argument parsing  
```rust
use std::env::args;
use cmdparsing::define; 


define! {
    Data; //name of the struct
    help: "usage: cmd [file(2)] [other]" //the usage of the command(shows when using -help, or --help(reserved flag))
    flags {
        t: bool = "w"|"h",  // a flag that uses a boolean
        f: String = "f", // a flag of type string
        o: String = "l" => [2], //a flag of type string with 2 entries
    };
    args {
        file: String => [2], //a positional argument with 2 entries(String)
        other: String, //a positional arument with one entry(String)
    };
    rest => more: String; //a rest argument that takes strings(takes the last arguments unless its broken by a flag)
}

fn main() {
    let d = Data::from(args().collect());
    println!("{:?}", d);
}

```

that example would generate this struct 

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
