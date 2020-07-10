# string_parser
## Rust string parsing crate
### Doc : https://docs.rs/string-parser/0.1.0/string_parser/
### Usage :
```Rust
extern crate string_parser;
use string_parser::string_parser; 
 
fn end_filter(c : char) -> bool{            
    if c == '\'' {
        return true;
        }
    else {
        return false;
        }   
}
 
fn callback(s : String){
    assert_eq!(String::from("foo"), s);
}
string_parser("./text", "'", end_filter, callback).unwrap();
```

