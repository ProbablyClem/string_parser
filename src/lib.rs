//! # string_parser
//! 
//! string_parser is a crate that find tokens in source files and parse the inside<br/> 

use std::rc::Rc;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn doc_test() {
        // let mut s1 = String::new();

        let callback = |s : String, l : usize, f : &str| { 
            println!("{} {} : {}", f, l, s);
            if s != String::from("foo"){
                panic!();
            }
            // s1 = s;
        };

        let mut s_p = Parser::new();
        s_p.add(String::from("'"), Rc::new(Box::from(end_filter)), Rc::new(Box::from(callback)));
        assert_eq!(s_p.parse("./text").unwrap(), ());

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

fn string_parser(path : &str, p : &Parser) -> Result<(), io::Error> {

    //for everykeyword know if we're inside the token
    let mut insides : HashMap<String, bool> = HashMap::new();
    for i in p.clone().get_keywords(){
        let b = false;
        insides.insert(i.clone(), b.clone());
    }

    let mut first : bool = true; // true is it's the first iteration

    //open the file and put it as a string into file_buf
    let mut string_buffer = String::new();
    let mut file_buf = String::new(); //the whole file as a String
    let mut line : usize = 0;
    let f = File::open(path)?;
    let mut f = BufReader::new(f);
    f.read_to_string(&mut file_buf)?;

    //create a buffer for every keyword 
    let mut buffers : HashMap<String, Vec<char>> = HashMap::new();

    for i in p.clone().get_keywords(){
        let buff : Vec<char> = vec![' '; i.len()];
        buffers.insert(i.clone(), buff);
    }
    
    //loop through every character of the file
    for c in file_buf.chars() {
        //count the line number
        if c == '\n' {
            line += 1;
        }

        for i in p.clone().get_keywords(){
            let mut buff : Vec<char> = (&buffers.get(&i).unwrap()).to_vec();
            let end_filter_hash = p.clone().get_end_filters();
            let end_filter = end_filter_hash.get(&i).unwrap();
            let cb_hm = &p.clone().get_callbacks();
            let callback = cb_hm.get(&i).unwrap();

            let mut y : usize = 0;
            while y < buff.len() -1 {
                buff[y] = buff[y+1];
                y += 1;
            }
            buff[y] = c;
            y = 0;
    
            if *insides.get(&i).unwrap() && !end_filter(buff.clone()){
                string_buffer.push(c);
            }
            else if *insides.get(&i).unwrap() && !first {
                insides.insert(i, false);
                // let s = string_buffer.pop();
                callback(string_buffer.clone(), line, path);
                string_buffer.clear();
            }
            else {
                while y < buff.len(){
                    // println!("buff[{}] : {}, text[{}] : {}", i, buff[i], i, text.chars().nth(i).unwrap());
                    if buff[y] != i.chars().nth(y).unwrap() {
                        break;
                    }
                    y += 1;
                }
                if y == i.len() {
                    insides.insert(i, false);
                    first = false;
                }
            }
            
        }
        
        for i in &p.clone().get_keywords(){
            if *insides.get(i).unwrap() {
                let cb_hm = &p.clone().get_callbacks();
                let cb = cb_hm.get(i).unwrap();
                cb(string_buffer.clone(), line +1, path);
            }
        }
        
        }
        
    Ok(())
}

///Main struct of the crate
/// # Example
/// ./text being "...'foo'..."
/// ```no_run
/// use std::rc::Rc;
/// extern crate string_parser;
/// use string_parser::Parser; 
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
/// let callback = |s : String, line : usize, file : &str| {
///     assert_eq!(String::from("foo"), s); 
/// };
/// let mut string_parser = Parser::new();
/// string_parser.add(String::from("'"), Rc::new(Box::from(end_filter)), Rc::new(Box::from(callback)));
/// string_parser.parse("./text");
 
#[derive(Clone)]
pub struct Parser{
    ///The list of keyword to parse
    pub keywords : Vec<String>,
    ///Filter funtions for each keyword (indexed by keyword)
    pub end_filters : HashMap<String, Rc<Box<dyn Fn(Vec<char>) -> bool>>>,
    ///Callback funtions for each keyword (indexed by keyword)
    pub callbacks : HashMap<String, Rc<Box<dyn Fn(String, usize, &str)>>>,
}


impl Parser {
    ///Instanciante a new empty Parser
    pub fn new() -> Parser {
        Parser {
            keywords : Vec::new(),
            end_filters : HashMap::new(),
            callbacks : HashMap::new(),
        }
    }

    ///Add a new token to parse
    /// # Arguments
    /// * `text` - the text to search
    /// * `end_filter` - the function called at each character to check if we're still within the token. Sould return true when out of the token.
    ///     </br>It receives the vector of characters inside the token
    ///      * Should return true when at the end of the token 
    /// # Example
    /// ```no_run
    /// fn end_filter(c : Vec<char>) -> bool{
    ///     if c.last().unwrap() == &'\'' {
    ///         println!("end filter");
    ///         return true;
    ///     }
    ///     else {
    ///         return false;
    ///     }
    /// }
    /// ```
    /// * `callback` - the function called when the text is exited. take the inside of the token as argument
    ///    </br>it should takes 3 arguments
    ///     * The text inside the token
    ///     * The line Number of the token
    ///     * The file
    /// # Example
    /// ```no_run
    /// let callback = |s : String, line : usize, file : String| {
    ///     println!("{} {} : {}", file, line, s);
    /// };
    /// ```
    
    pub fn add(&mut self, keyword : String, end_filter : Rc<Box<dyn Fn(Vec<char>) -> bool>>, callback : Rc<Box<dyn Fn(String, usize, &str)>>) -> &Parser{
        
        self.keywords.push(keyword.clone());
        self.end_filters.insert(keyword.clone(), end_filter);
        self.callbacks.insert(keyword.clone(), callback);
        self
    }

    pub fn get_keywords(self) -> Vec<String> {
        self.keywords.clone()
    }

    pub fn get_end_filters(self) -> HashMap<String, Rc<Box<dyn Fn(Vec<char>) -> bool>>> {
        self.end_filters
    }

    pub fn get_callbacks(self) -> HashMap<String, Rc<Box<dyn Fn(String, usize, &str)>>> {
        self.callbacks
    }

    ///Open the file and parse every token of the Parser
    pub fn parse(&self, path : &str) -> Result<(), io::Error> {
        string_parser(path, &self).expect("failed to open file");
        Ok(())
    }
}