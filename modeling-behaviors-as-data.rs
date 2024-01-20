#[warn(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Location<'a>(&'a str);
#[derive(Debug, Clone, PartialEq)]
pub enum MusicGenre {
    HeavyMetal,
    Pop,
    HardRock,
}

#[derive(Debug, Clone)]
pub enum YearsActive {
    StillActive { since: u16 },
    ActiveBetween { start: u16, end: u16 },
}

pub enum SearchCondition<'a> {
    SearchByGenre { genres: Vec<MusicGenre> },
    SearchByOrigin { locations: Vec<Location<'a>> },
    SearchByActiveYears { start: u16, end: u16 },
}

#[derive(Debug, Clone)]
pub struct Artist<'a> {
    name: &'a str,             // 乐队名
    genre: MusicGenre,         // 流派
    origin: Location<'a>,      // 发源地
    years_active: YearsActive, // 活跃时段
}

pub fn was_artist_active(artist: &Artist, year_start: u16, year_end: u16) -> bool {
    match artist.years_active {
        YearsActive::StillActive { since } => since <= year_end,
        YearsActive::ActiveBetween { start, end } => start <= year_end && end >= year_start,
    }
}

// search through a list of music artists.
// Each search should support different combination of conditions:
// by genre, by origin, and by period in which they were active.
pub fn search_artists<'a>(
    artists: Vec<Artist<'a>>,
    required_conditions: Vec<SearchCondition>,
) -> Vec<Artist<'a>> {
    artists
        .into_iter()
        .filter(|artist| {
            required_conditions.iter().all(|condition| match condition {
                SearchCondition::SearchByGenre { genres } => genres.contains(&artist.genre),
                SearchCondition::SearchByOrigin { locations } => locations.contains(&artist.origin),
                SearchCondition::SearchByActiveYears { start, end } => {
                    was_artist_active(artist, *start, *end)
                }
            })
        })
        .collect::<Vec<Artist>>()
}

fn main() {
    // use these three artists in tests
    // - Metallica, Heavy Metal, U.S., 1981-now
    // - Led Zeppelin, Hard Rock, England, 1968-1980
    // - Bee Gees, Pop, England, 1958-2003
    let artists = vec![
        Artist {
            name: "Metallica",
            genre: MusicGenre::HeavyMetal,
            origin: Location("U.S."),
            years_active: YearsActive::StillActive { since: 1981 },
        },
        Artist {
            name: "Led Zeppelin",
            genre: MusicGenre::HardRock,
            origin: Location("England"),
            years_active: YearsActive::ActiveBetween {
                start: 1968,
                end: 1980,
            },
        },
        Artist {
            name: "Bee Gees",
            genre: MusicGenre::Pop,
            origin: Location("England"),
            years_active: YearsActive::ActiveBetween {
                start: 1958,
                end: 2003,
            },
        },
    ];

    // Zero condition
    let s0 = search_artists(artists.clone(), vec![]);

    // Search for pop artists from England active between 1950 and 2022
    let s1 = search_artists(
        artists.clone(),
        vec![
            SearchCondition::SearchByGenre {
                genres: vec![MusicGenre::Pop],
            },
            SearchCondition::SearchByActiveYears {
                start: 1950,
                end: 2022,
            },
            SearchCondition::SearchByOrigin {
                locations: vec![Location("England")],
            },
        ],
    );
    for s in s1 {
        println!("{:?}", s);
    }

    // Search for artists from England active between 1950 and 2022
    let s1 = search_artists(
        artists.clone(),
        vec![
            SearchCondition::SearchByActiveYears {
                start: 1950,
                end: 2022,
            },
            SearchCondition::SearchByOrigin {
                locations: vec![Location("England")],
            },
        ],
    );
    for s in s1 {
        println!("{:?}", s);
    }

    // Search for artists active between 1950 and 1979
    let s1 = search_artists(
        artists.clone(),
        vec![SearchCondition::SearchByActiveYears {
            start: 1950,
            end: 1979,
        }],
    );
    for s in s1 {
        println!("{:?}", s);
    }

    // Search for artists active between 1981 and 1984
    let s1 = search_artists(
        artists.clone(),
        vec![SearchCondition::SearchByActiveYears {
            start: 1981,
            end: 1984,
        }],
    );
    for s in s1 {
        println!("{:?}", s);
    }

    // Search for heavy-metal artists active between 2019 and 2022
    let s1 = search_artists(
        artists.clone(),
        vec![
            SearchCondition::SearchByGenre {
                genres: vec![MusicGenre::HeavyMetal],
            },
            SearchCondition::SearchByActiveYears {
                start: 2019,
                end: 2022,
            },
        ],
    );
    for s in s1 {
        println!("{:?}", s);
    }

    // Search for artists from the U.S. active between 1950 and 1959
    let s1 = search_artists(
        artists.clone(),
        vec![
            SearchCondition::SearchByActiveYears {
                start: 1950,
                end: 1959,
            },
            SearchCondition::SearchByOrigin {
                locations: vec![Location("U.S.")],
            },
        ],
    );
    for s in s1 {
        println!("{:?}", s);
    }

    // Search for artists without any conditions
    let s1 = search_artists(
        artists.clone(),
        vec![SearchCondition::SearchByActiveYears {
            start: 2019,
            end: 2022,
        }],
    );
    for s in s1 {
        println!("{:?}", s);
    }

    let search_conditions = vec![
        SearchCondition::SearchByGenre {
            genres: vec![MusicGenre::Pop],
        },
        SearchCondition::SearchByOrigin {
            locations: vec![Location("England")],
        },
        SearchCondition::SearchByActiveYears {
            start: 1950,
            end: 2022,
        },
    ];
    let s1 = search_artists(artists.clone(), search_conditions);
    for s in s1 {
        println!("{:?}", s);
    }
}