use std::io;
use chrono::prelude::*;

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

pub fn edit_option_line(e: &Option<String>, n: &str) -> Option<String> {
    match e {
        Some(s) => Some(edit_line(&s, n)),
        None => {
            let l = edit_line(&String::new(), n);
            if l.is_empty() { None } else { Some(l) }
        }
    }
}

pub fn edit_datetime(e: &NaiveDateTime, n: &str) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(
            &edit_line(&e.to_string(), n),
            "%Y-%m-%d %H:%M:%S").unwrap_or(e.clone())
}

pub fn edit_option_datetime(e: &Option<NaiveDateTime>, n: &str) -> Option<NaiveDateTime> {
    match e {
        Some(s) => Some(edit_datetime(&s, n)),
        None => Some(edit_datetime(&Utc::now().naive_utc(), n)), 
    }
}

pub fn edit_number(e: i32, n: &str) -> i32 {
        println!("{}: {}", &n, e);
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

pub fn edit_option_number(e: Option<i32>, n: &str) -> Option<i32> {
    match e {
        Some(s) => Some(edit_number(s, n)),
        None => Some(edit_number(0, n)), 
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
    
pub fn edit_option_text(e: &Option<String>, n: &str) -> Option<String> {
    match e {
        Some(s) => Some(edit_text(&s, n)),
        None => {
            let l = edit_text(&String::new(), n);
            if l.is_empty() { None } else { Some(l) }
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
