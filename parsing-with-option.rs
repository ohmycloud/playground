#![feature(let_chains)]
#[warn(dead_code)]
#[derive(Debug, PartialEq)]

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

pub fn validate_length(start: u16, end: u16, min_length: u16) -> Option<u16> {
    if end - start >= min_length {
        Some(end - start)
    } else {
        None
    }
}

pub fn parse_long_event(name: String, start: u16, end: u16, min_length: u16) -> Option<Event> {
    if let Some(name) = validate_name(name) && 
       let Some(end) = validate_end(end) && 
       let Some(start) = validate_start(start, end) && 
       let Some(min_length) = validate_length(start, end, min_length) {
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
    let e = parse_long_event("Apollo Program".to_string(), 1961, 1972, 10);
    assert_eq!(e, Some(Event { name: "Apollo Program".to_string(), start: 1961, end: 1972 }));
    
    let e = parse_long_event("World War II".to_string(), 1939, 1945, 10);
    assert_eq!(e, None);
}