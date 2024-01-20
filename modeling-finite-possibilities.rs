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
pub struct PeriodInYears(u16, Option<u16>);

#[derive(Debug, Clone)]
pub struct Artist<'a> {
    name: &'a str,                // 乐队名
    genre: MusicGenre,            // 流派
    origin: Location<'a>,         // 发源地
    years_active: PeriodInYears,  // 活跃时段
}

// search through a list of music artists.
// Each search should support different combination of conditions:
// by genre, by origin, and by period in which they were active.
pub fn search_artists<'a>(
    artists: Vec<Artist<'a>>,
    genres: Vec<MusicGenre>,
    locations: Vec<Location>,
    search_by_active_years: bool,
    active_after: u16,
    active_before: u16
) -> Vec<Artist<'a>> {
    artists.into_iter().filter(|artist|
        (genres.is_empty() || genres.contains(&artist.genre)) &&
        (locations.is_empty() || locations.contains(&artist.origin)) &&
        (!search_by_active_years || (
            (artist.years_active.1.is_some_and(|x| x >= active_after)) &&
            (artist.years_active.0 <= active_before)
        ))
    ).collect::<Vec<Artist>>()
}


fn main() {
    // use these three artists in tests
    // - Metallica, Heavy Metal, U.S., 1981-now
    // - Led Zeppelin, Hard Rock, England, 1968-1980
    // - Bee Gees, Pop, England, 1958-2003
    let artists = vec![
        Artist {name: "Metallica", genre: MusicGenre::HeavyMetal, origin: Location("U.S."), years_active: PeriodInYears(1981, None) }, 
        Artist {name: "Led Zeppelin", genre: MusicGenre::HardRock, origin: Location("England"), years_active: PeriodInYears(1968, Some(1980)) },
        Artist {name: "Bee Gees", genre: MusicGenre::Pop, origin: Location("England"), years_active: PeriodInYears(1958, Some(2003)) },
    ];

    // Search for pop artists from England active between 1950 and 2022
    let s1  = search_artists(artists.clone(), vec![MusicGenre::Pop], vec![Location("England")], true, 1950, 2022);
    for s in s1 {
        println!("{:?}", s);
    }
    
    // Search for artists from England active between 1950 and 2022
    let s1 = search_artists(artists.clone(), vec![], vec![Location("England")], true, 1950, 2022);
    for s in s1 {
        println!("{:?}", s);
    }
    
    // Search for artists active between 1950 and 1979
    let s1 = search_artists(artists.clone(), vec![], vec![], true, 1950, 1979);
    for s in s1 {
        println!("{:?}", s);
    }
    
    // Search for artists active between 1981 and 1984
    let s1 = search_artists(artists.clone(), vec![], vec![], true, 1981, 1984);
    for s in s1 {
        println!("{:?}", s);
    }

    // Search for heavy-metal artists active between 2019 and 2022
    let s1 = search_artists(artists.clone(), vec![MusicGenre::HeavyMetal], vec![], false, 2019, 2022);
    for s in s1 {
        println!("{:?}", s);
    }
    
    // Search for artists from the U.S. active between 1950 and 1959
    let s1 = search_artists(artists.clone(), vec![], vec![Location("U.S.")], true, 1950, 1959);
    for s in s1 {
        println!("{:?}", s);
    }
    
    // Search for artists without any conditions
    // It's not a mistake, but including years here feels misleading, doesn't it?
    let s1 = search_artists(artists.clone(), vec![], vec![], false, 2019, 2022); 
    for s in s1 {
        println!("{:?}", s);
    }
}