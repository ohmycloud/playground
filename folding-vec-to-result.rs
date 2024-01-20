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

pub fn extract_year_start(raw_show: &str) -> Result<u16, String> {
    if let Some(bracket_open)  = raw_show.find('(') &&
       let Some(dash)          = raw_show.find('-') {
           if dash > bracket_open + 1  {
            let year_start = &raw_show[bracket_open + 1 .. dash];
            if let Ok(year_start) = year_start.parse::<u16>() {
                Ok(year_start)
            } else { Err(format!("Can't parse year start {}", year_start)) }
            } else { Err(format!("dash is before bracket open")) }
       }
    else { Err(format!("Can't parse dash and bracket open")) }
}

pub fn extract_year_end(raw_show: &str) -> Result<u16, String> {
    if let Some(dash)          = raw_show.find('-') &&
       let Some(bracket_close) = raw_show.find(')') {
           if bracket_close > dash + 1 {
              let year_end   = &raw_show[dash + 1 .. bracket_close];
              if let Ok(year_end) = year_end.parse::<u16>() { Ok(year_end) } else { Err(format!("Can't parse year end {}", year_end)) }
            } else { Err(format!("bracket close is before dash")) }
       } else { Err(format!("dash and bracket close parse error")) }
}

pub fn extract_name(raw_show: &str) -> Result<&str, String> {
    if let Some(bracket_open) = raw_show.find('(') {
        Ok(&raw_show[0..bracket_open].trim())
    } else {
        Err(format!("Can't extract name from {}", raw_show))
    }
}

pub fn add_or_resign<'a>(
    parsed_shows: Result<Vec<TvShow<'a>>, String>,
    new_parsed_show: Result<TvShow<'a>, String>) 
  -> Result<Vec<TvShow<'a>>, String> {
  let mut v: Vec<TvShow> = Vec::new();
  if let Ok(shows) = parsed_shows &&
     let Ok(show) = new_parsed_show {
     for s in shows.into_iter() {
         v.push(s);
     }
     v.push(show);
     Ok(v)
  } else {
      Err(format!("Can't fold"))
  }
}

pub fn parse_show(raw_show: &str) -> Result<TvShow, String> {
    let name = extract_name(raw_show);
    let year_start = extract_year_start(raw_show);
    let year_end = extract_year_end(raw_show);

    if let Ok(name) = name &&
       let Ok(year_start) = year_start &&
       let Ok(year_end) = year_end {
           Ok(TvShow {title: name, start: year_start, end: year_end})
    } else { 
      Err(format!("Can't parse '{}'", raw_show)) 
    }
}


// Folding a Vec of Results into an Result of a Vec
pub fn parse_shows(raw_shows: Vec<&str>) -> Result<Vec<TvShow>, String> {
    let initial_result: Result<Vec<TvShow>, String> = Ok(vec![]);
    raw_shows
      .iter()
      .map(|x| parse_show(x))
      .collect::<Vec<Result<TvShow, String>>>()
      .into_iter()
      .inspect(|x|println!("{:?}", x))
      .fold(initial_result, add_or_resign)
}

fn main() {
    let raw_shows = vec![
        "Breaking Bad (2008-2013)",
        "The Wire (2002-2008)",
        "Mad Men (2007-2015)",
        "Mad Men (-2015)",
        "(2002- N/A ) The Wire",
    ];
    
    if let Err(err_end) = extract_year_end("(2002- N/A ) The Wire") {
        println!("{:?}", err_end);
    }
    
    if let Err(err_start) = extract_year_start("Mad Men (-2015)") {
        println!("{:?}", err_start);
    }
    
    if let Err(err_show) = parse_show("Mad Men (-2015)") {
        println!("{:?}", err_show);
    }
    
    if let Err(parsed_shows) = parse_shows(raw_shows) {
        println!("{:?}", parsed_shows);
    } else {
        println!("{:?}", "ok");
    }
}