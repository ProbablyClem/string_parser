//! # string_parser
//! 
//! string_parser is a crate that find tokens in source files and parse the inside<br/> 

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn doc_test() {
        let mut s1 = String::new();

        let callback = |s : String| { 
            if s != String::from("foo"){
                panic!();
            }
            s1 = s;
        };


        assert_eq!(string_parser("./text", "'", end_filter, callback).unwrap(), ());

        fn end_filter(c : Vec<char>) -> bool{
            for char in &c {
                print!("{}", char);
            }
            print!("\n");
            if c.last().unwrap() == &'\'' {
                println!("end filter");
                return true;
            }
            else {
                return false;
            }
        }
    }

    

    
}

//path : chemin d'acces du fichier
//txt : text to find
//filter function pointer pour la fin du string
//callback methode appelé a la fin
/// # Arguments
/// * `path` - the path to the file to search from
/// * `text` - the text to search
/// * `end_filter` - the function called at each character to check if we're still within the token. Sould return true when out of the token.
/// * `callback` - the function called when the text is exited. take the inside of the token as argument
/// # Examples
/// ./text being "...'foo'..."
/// ```no_run
/// extern crate string_parser;
/// use string_parser::string_parser; 
/// 
/// fn end_filter(c : Vec<char>) -> bool{            
///     if c.last().unwrap()== &'\'' {
///         return true;
///         }
///     else {
///         return false;
///         }   
/// }
/// //can also use closures
/// let callback = |s : String| {
///     assert_eq!(String::from("foo"), s); 
/// };
/// 
/// string_parser("./text", "'", end_filter, callback).unwrap();
pub fn string_parser(path : &str,text : &str, end_filter : impl Fn(Vec<char>) -> bool ,mut callback : impl FnMut(String)) -> Result<(), io::Error> {
    //open the file and put it as a string into file_buf
    let mut inside : bool = false; // true if the cursor is inside the statement
    let mut first : bool = true; // true is it's the first iteration
    let mut string_buffer = String::new();

    let mut file_buf = String::new(); //the whole file as a String
    let f = File::open(path)?;
    let mut f = BufReader::new(f);
    f.read_to_string(&mut file_buf)?;

    let mut buff : Vec<char> = vec![' '; text.len()];
    //loop through every character of the file
    for c in file_buf.chars() {
        let mut i : usize = 0;
        while i < buff.len() -1 {
            buff[i] = buff[i+1];
            i += 1;
        }
        buff[i] = c;
        i = 0;

        if inside && !end_filter(buff.clone()){
            string_buffer.push(c);
        }
        else if inside && !first {
            inside = false;
            // let s = string_buffer.pop();
            callback(string_buffer.clone());
        }
        else {
            while i < buff.len(){
                // println!("buff[{}] : {}, text[{}] : {}", i, buff[i], i, text.chars().nth(i).unwrap());
                if buff[i] != text.chars().nth(i).unwrap() {
                    break;
                }
                i += 1;
            }
            if i == text.len() {
                inside = true;
                first = false;
            }
        }
        

        

        

        


    }
    Ok(())
}