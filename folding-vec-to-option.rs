#![feature(let_chains)]
#[warn(dead_code)]

#[derive(Debug, PartialEq)]
pub struct TvShow<'a> {
    title: &'a str,
    start: u16,
    end: u16,
}

pub fn sort_shows(mut shows: Vec<TvShow>) -> Vec<TvShow> {
    shows.sort_by(|t1, t2| (t2.end - t2.start).cmp(&(t1.end - t1.start)));
    shows
}

pub fn extract_year_start(raw_show: &str) -> Option<u16> {
    if let Some(bracket_open)  = raw_show.find('(') &&
       let Some(dash)          = raw_show.find('-') {
           if dash > bracket_open + 1  {
            let year_start = &raw_show[bracket_open + 1 .. dash];
            if let Ok(year_start) = year_start.parse::<u16>() {
                Some(year_start)
            } else { None }
            } else { None }
       }
    else { None }
}

pub fn extract_year_end(raw_show: &str) -> Option<u16> {
    if let Some(dash)          = raw_show.find('-') &&
       let Some(bracket_close) = raw_show.find(')') {
           if bracket_close > dash + 1 {
              let year_end   = &raw_show[dash + 1 .. bracket_close];
              if let Ok(year_end) = year_end.parse::<u16>() { Some(year_end) } else { None }
            } else { None }
       } else { None }
}

pub fn extract_name(raw_show: &str) -> Option<&str> {
    if let Some(bracket_open) = raw_show.find('(') {
        Some(&raw_show[0..bracket_open].trim())
    } else {
        None
    }
}

pub fn add_or_resign<'a>(
    parsed_shows: Option<Vec<TvShow<'a>>>,
    new_parsed_show: Option<TvShow<'a>>) 
  -> Option<Vec<TvShow<'a>>> {
  let mut v: Vec<TvShow> = Vec::new();
  if let Some(shows) = parsed_shows &&
     let Some(show) = new_parsed_show {
     for s in shows.into_iter() {
         v.push(s);
     }
     v.push(show);
     Some(v)
  } else {
      None
  }
}

pub fn parse_show(raw_show: &str) -> Option<TvShow> {
    let name = extract_name(raw_show);
    let year_start = extract_year_start(raw_show);
    let year_end = extract_year_end(raw_show);

    if let Some(name) = name &&
       let Some(year_start) = year_start &&
       let Some(year_end) = year_end {
           Some(TvShow {title: name, start: year_start, end: year_end})
    } else { 
      None 
    }
}


// Folding a Vec of Options into an Option of a Vec
pub fn parse_shows(raw_shows: Vec<&str>) -> Option<Vec<TvShow>> {
    let initial_result: Option<Vec<TvShow>> = Some(vec![]);
    raw_shows
      .iter()
      .map(|x| parse_show(x))
      .collect::<Vec<Option<TvShow>>>()
      .into_iter()
      .inspect(|x|println!("{:?}", x))
      .fold(initial_result, add_or_resign)
}

fn main() {
    let raw_shows = vec![
        "Breaking Bad (2008-2013)",
        "The Wire (2002-2008)",
        "Mad Men (2007-2015)",
    ];
    
    if let Some(parsed_shows) = parse_shows(raw_shows) {
        println!("{:?}", parsed_shows);
    } else {
        println!("{:?}", "Error");
    }
}