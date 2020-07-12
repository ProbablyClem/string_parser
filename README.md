# string_parser
## Rust string parsing crate
### Doc : https://crates.io/crates/string-parser
### Usage :
```Rust
use std::rc::Rc;
extern crate string_parser;
use string_parser::Parser; 
 
fn end_filter(c : Vec<char>) -> bool{            
    if c.last().unwrap()== &'\'' {
        return true;
        }
    else {
        return false;
        }   
}

//can also use closures
let callback = |s : String, line : usize, file : &str| {
    assert_eq!(String::from("foo"), s); 
};

let mut string_parser = Parser::new();

string_parser.add(String::from("'"), Rc::new(Box::from(end_filter)), Rc::new(Box::from(callback)));
string_parser.parse("./text");
```

