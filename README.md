# string_parser
## Rust string parsing crate
### Doc : https://crates.io/crates/string-parser
### Usage :
```Rust
extern crate string_parser;
use string_parser::string_parser; 
 
fn end_filter(c : Vec<char>) -> bool{            
    if c.last().unwrap()== &'\'' {
        return true;
        }
    else {
        return false;
        }   
}
//can also use closures
let callback = |s : String| {
    assert_eq!(String::from("foo"), s); 
};

string_parser("./text", "'", end_filter, callback).unwrap();
```

