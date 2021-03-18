use std::io;

pub fn edit_line(e: &String, n: &str) -> String {
        println!("{}: {}", &n, &e);
        let mut input = String::new();
        
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim().is_empty() {
                    println!("{}: {}", &n, e);
                    e.clone()
                } else {
                    println!("{}: {}", &n, input);
                    input.trim().to_string()
                }
            },
            Err(error) => {
                println!("error: {}", error);
                e.clone()
            }
        }
    }

pub fn edit_number(e: &i32, n: &str) -> i32 {
        println!("{}: {}", &n, &e);
        let mut input = String::new();
        
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim().is_empty() {
                    println!("{}: {}", &n, e);
                    e.clone()
                } else {
                    println!("{}: {}", &n, input);
                    match input.trim().parse::<i32>() {
                        Ok(i) => i,
                        Err(..) => e.clone()
                    }
                }
            },
            Err(error) => {
                println!("error: {}", error);
                e.clone()
            }
        }
    }

pub fn edit_text(e: &String, n: &str) -> String {
        let edited = edit::edit(e);
        
        match edited {
            Ok(s) => {
                println!("{}:\n{}", &n, s);
                s
            },
            Err(error) => {
                println!("error: {}", error);
                e.clone()
            }
        }
    }

pub fn edit_bool(e: bool, n: &str) -> bool {
        println!("{}: {}", &n, &e);
        let mut input = String::new();
        
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim().is_empty() {
                    println!("{}: {}", &n, e);
                    e
                } else {
                    let t = input.trim();
                    if t == "1" || t == "t" || t == "y" {
                        true
                    } else {
                        false
                    }
                }
            },
            Err(error) => {
                println!("error: {}", error);
                e
            }
        }
    }
