#[warn(dead_code)]
/// Implement a music artist catalogue that 
/// will help us find artists by genres, their locations, or years they were active.
/// Each artist will have a name, a main genre, and an origin(location), year their career started
/// and a year they stopped performing(if they are not still active).
/// We just need to define a struct.

#[derive(Debug, Clone, PartialEq)]
pub struct Location<'a>(&'a str);
#[derive(Debug, Clone, PartialEq)]
pub enum MusicGenre {
    HeavyMetal,
    Pop,
    HardRock
}

#[derive(Debug, Clone)]
pub enum YearsActive {
    StillActive   { since: u16 },
    ActiveBetween { start: u16, end: u16 }
}


#[derive(Debug, Clone)]
pub struct Artist<'a> {
    name: &'a str,                // 乐队名
    genre: MusicGenre,            // 流派
    origin: Location<'a>,         // 发源地
    years_active: YearsActive,    // 活跃时段
}

pub fn active_length(artist: &Artist, current_year: u16) -> u16 {
    match artist.years_active {
        YearsActive::StillActive   {since}      => current_year - since,
        YearsActive::ActiveBetween {start, end} => end - start
    }
}

fn main() {
    // use these three artists in tests
    // - Metallica, Heavy Metal, U.S., 1981-now
    // - Led Zeppelin, Hard Rock, England, 1968-1980
    // - Bee Gees, Pop, England, 1958-2003
    let artists = vec![
        Artist {name: "Metallica",    genre: MusicGenre::HeavyMetal, origin: Location("U.S."),    years_active: YearsActive::StillActive   {since: 1981}}, 
        Artist {name: "Led Zeppelin", genre: MusicGenre::HardRock,   origin: Location("England"), years_active: YearsActive::ActiveBetween {start: 1968, end: 1980}},
        Artist {name: "Bee Gees",     genre: MusicGenre::Pop,        origin: Location("England"), years_active: YearsActive::ActiveBetween { start: 1958, end: 2003}},
    ];

    for artist in artists.iter() {
        println!("{:?}", active_length(artist, 2023));
    }
}