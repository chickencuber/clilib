# CLILIB
a macro to do command line argument parsing  
```rust
define! {
    Data; //name of the struct
    flags { //any command line flags go here
        t: bool = "w"|"h",  // this creates a feild on the struct called t, and makes -w and -h set it
        f: String = "f", //supports more than just bools, and unlike bools requires an argumnet
    };
    args {
        file: String, // a single argument
    };
}

```
