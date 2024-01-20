#![feature(let_chains)]
#[warn(dead_code)]
#[derive(Debug)]

pub struct Event {
    name: String,
    start: u16,
    end: u16,
}

pub fn validate_name(name: String) -> Option<String> {
    if name.len() > 0 {
        Some(name)
    } else {
        None
    }
}

pub fn validate_end(end: u16) -> Option<u16> {
    if end < 3000 {
        Some(end)
    } else {
        None
    }
}

pub fn validate_start(start: u16, end: u16) -> Option<u16> {
    if start <= end {
        Some(start)
    } else {
        None
    }
}

pub fn parse(name: String, start: u16, end: u16) -> Option<Event> {
    if let Some(name) = validate_name(name) && 
       let Some(end) = validate_end(end) && 
       let Some(start) = validate_start(start, end) {
            Some(Event {
                name,
                start,
                end
            })
    } else { 
        None
    }
}

fn main() {
    let e = parse("Apollo Program".to_string(), 1961, 1972);
    println!("{:?}", e);
}