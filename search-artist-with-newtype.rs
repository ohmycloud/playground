#[warn(dead_code)]
/// Implement a music artist catalogue that 
/// will help us find artists by genres, their locations, or years they were active.
/// Each artist will have a name, a main genre, and an origin(location), year their career started
/// and a year they stopped performing(if they are not still active).
/// We just need to define a struct.

#[derive(Debug, Clone, PartialEq)]
pub struct Location<'a>(&'a str);
#[derive(Debug, Clone, PartialEq)]
pub struct Genre<'a>(&'a str);
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct YearsActiveStart(u16);
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct YearsActiveEnd(u16);

#[derive(Debug, Clone)]
pub struct Artist<'a> {
    name: &'a str,                        // 乐队名
    genre: Genre<'a>,                     // 流派
    origin: Location<'a>,                 // 发源地
    years_active_start: YearsActiveStart, // 活跃年份
    is_active: bool,                      // 是否活跃
    years_active_end: YearsActiveEnd      // 停止活跃年份
}

// search through a list of music artists.
// Each search should support different combination of conditions:
// by genre, by origin, and by period in which they were active.
pub fn search_artists<'a>(
    artists: Vec<Artist<'a>>,
    genres: Vec<Genre>,
    locations: Vec<Location>,
    search_by_active_years: bool,
    active_after: YearsActiveStart,
    active_before: YearsActiveEnd
) -> Vec<Artist<'a>> {
    artists.into_iter().filter(|artist|
        (genres.is_empty() || genres.contains(&artist.genre)) &&
        (locations.is_empty() || locations.contains(&artist.origin)) &&
        (!search_by_active_years || (
            (artist.is_active || artist.years_active_end.0 >= active_after.0) &&
            (artist.years_active_start.0 <= active_before.0)))
    ).collect::<Vec<Artist>>()
}


fn main() {
    // use these three artists in tests
    // - Metallica, Heavy Metal, U.S., 1981-now
    // - Led Zeppelin, Hard Rock, England, 1968-1980
    // - Bee Gees, Pop, England, 1958-2003
    let artists = vec![
        Artist {name: "Metallica", genre: Genre("Heavy Metal"), origin: Location("U.S."), years_active_start: YearsActiveStart(1981), is_active: true, years_active_end: YearsActiveEnd(0)}, 
        Artist {name: "Led Zeppelin", genre: Genre("Hard Rock"), origin: Location("England"), years_active_start: YearsActiveStart(1968), is_active: false, years_active_end: YearsActiveEnd(1980)},
        Artist {name: "Bee Gees", genre: Genre("Pop"), origin: Location("England"), years_active_start: YearsActiveStart(1958), is_active: false, years_active_end: YearsActiveEnd(2003)},
    ];

    // Search for pop artists from England active between 1950 and 2022
    let s1  = search_artists(artists.clone(), vec![Genre("Pop")], vec![Location("England")], true, YearsActiveStart(1950), YearsActiveEnd(2022));
    for s in s1 {
        println!("{:?}", s);
    }
    
    // Search for artists from England active between 1950 and 2022
    let s1 = search_artists(artists.clone(), vec![], vec![Location("England")], true, YearsActiveStart(1950), YearsActiveEnd(2022));
    for s in s1 {
        println!("{:?}", s);
    }
    
    // Search for artists active between 1950 and 1979
    let s1 = search_artists(artists.clone(), vec![], vec![], true, YearsActiveStart(1950), YearsActiveEnd(1979));
    for s in s1 {
        println!("{:?}", s);
    }
    
    // Search for artists active between 1981 and 1984
    let s1 = search_artists(artists.clone(), vec![], vec![], true, YearsActiveStart(1981), YearsActiveEnd(1984));
    for s in s1 {
        println!("{:?}", s);
    }

    // Search for heavy-metal artists active between 2019 and 2022
    let s1 = search_artists(artists.clone(), vec![Genre("Heavy Metal")], vec![], false, YearsActiveStart(2019), YearsActiveEnd(2022));
    for s in s1 {
        println!("{:?}", s);
    }
    
    // Search for artists from the U.S. active between 1950 and 1959
    let s1 = search_artists(artists.clone(), vec![], vec![Location("U.S.")], true, YearsActiveStart(1950), YearsActiveEnd(1959));
    for s in s1 {
        println!("{:?}", s);
    }
    
    // Search for artists without any conditions
    // It's not a mistake, but including years here feels misleading, doesn't it?
    let s1 = search_artists(artists.clone(), vec![], vec![], false, YearsActiveStart(2019), YearsActiveEnd(2022)); 
    for s in s1 {
        println!("{:?}", s);
    }
}