#[warn(dead_code)]
/// Implement a music artist catalogue that 
/// will help us find artists by genres, their locations, or years they were active.
/// Each artist will have a name, a main genre, and an origin(location), year their career started
/// and a year they stopped performing(if they are not still active).
/// We just need to define a struct.

#[derive(Debug, Clone)]
pub struct Artist<'a> {
    name: &'a str,   // 乐队名
    genre: &'a str,  // 流派
    origin: &'a str, // 发源地
    years_active_start: u16, // 活跃年份
    is_active: bool, // 是否活跃
    years_active_end: u16 // 停止活跃年份
}

// search through a list of music artists.
// Each search should support different combination of conditions:
// by genre, by origin, and by period in which they were active.
pub fn search_artists<'a>(
    artists: Vec<Artist<'a>>,
    genres: Vec<&'a str>,
    locations: Vec<&'a str>,
    search_by_active_years: bool,
    active_after: u16,
    active_before: u16
) -> Vec<Artist<'a>> {
    artists.into_iter().filter(|artist|
        (genres.is_empty() || genres.contains(&artist.genre)) &&
        (locations.is_empty() || locations.contains(&artist.origin)) &&
        (!search_by_active_years || (
            (artist.is_active || artist.years_active_end >= active_after) &&
            (artist.years_active_start <= active_before)))
    ).collect::<Vec<Artist>>()
}


fn main() {
    let artists = vec![
        Artist {name: "Metallica", genre: "Heavy Metal", origin: "U.S.", years_active_start: 1981, is_active: true, years_active_end: 0}, 
        Artist {name: "Led Zeppelin", genre: "Hard Rock", origin: "England", years_active_start: 1968, is_active: false, years_active_end: 1980},
        Artist {name: "Bee Gees", genre: "Pop", origin: "England", years_active_start: 1958, is_active: false, years_active_end: 2003},
    ];

    // Search for pop artists from England active between 1950 and 2022
    search_artists(artists.clone(), vec!["Pop"], vec!["England"], true, 1950, 2022);
    // Search for artists from England active between 1950 and 2022
    search_artists(artists.clone(), vec![], vec!["England"], true, 1950, 2022);
    // Search for artists active between 1950 and 1979
    search_artists(artists.clone(), vec![], vec![], true, 1981, 2003);
    // Search for artists active between 1981 and 1984
    search_artists(artists.clone(), vec![], vec!["U.S."], false, 0, 0);
    // Search for heavy-metal artists active between 2019 and 2022
    search_artists(artists.clone(), vec![], vec![], false, 2019, 2022);
    // Search for artists from the U.S. active between 1950 and 1959
    search_artists(artists.clone(), vec![], vec!["U.S."], true, 1950, 1959);
    // Search for artists without any conditions
    search_artists(artists.clone(), vec![], vec![], false, 2019, 2022); 
}
