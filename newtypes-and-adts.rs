#[warn(dead_code)]
use std::vec;

#[derive(Debug, Clone, PartialEq)]
pub struct Location<'a>(&'a str);
#[derive(Debug, Clone, PartialEq)]
pub enum MusicGenre {
    HeavyMetal,
    Pop,
    HardRock,
}

pub enum SearchCondition<'a> {
    SearchByGenre { genres: Vec<MusicGenre> },
    SearchByOrigin { locations: Vec<Location<'a>> },
    SearchByActiveYears { period: PeriodInYears },
    SearchByActiveLength { how_long: u16, until: u16 },
}

// Both active and inactive artists can have previous periods of inactivity.
#[derive(Debug, Clone)]
pub enum YearsActive {
    StillActive {
        since: u16,
        previous_periods: Vec<PeriodInYears>,
    },
    ActiveInPast {
        periods: Vec<PeriodInYears>,
    },
}

#[derive(Debug, Clone)]
pub struct PeriodInYears {
    start: u16,
    end: u16,
}

#[derive(Debug, Clone)]
pub struct Artist<'a> {
    name: &'a str,             // 乐队名
    genre: MusicGenre,         // 流派
    origin: Location<'a>,      // 发源地
    years_active: YearsActive, // 活跃时段
}

// Check if checked_period overlaps with given periods.
pub fn period_overlaps_with_periods(
    checked_period: PeriodInYears,
    periods: Vec<PeriodInYears>,
) -> bool {
    periods
        .iter()
        .any(|x| x.start <= checked_period.end && x.end >= checked_period.start)
}

pub fn was_artist_active(artist: Artist, search_periods: PeriodInYears) -> bool {
    match artist.years_active {
        YearsActive::StillActive {
            since,
            previous_periods,
        } => {
            since <= search_periods.end
                || period_overlaps_with_periods(search_periods, previous_periods)
        }
        YearsActive::ActiveInPast { periods } => {
            period_overlaps_with_periods(search_periods, periods)
        }
    }
}

// 活跃时长
pub fn active_length(artist: Artist, current_year: u16) -> u16 {
    let periods = match artist.years_active {
        YearsActive::StillActive {
            since,
            previous_periods,
        } => previous_periods
            .into_iter()
            .chain(vec![PeriodInYears {
                start: since,
                end: current_year,
            }])
            .collect::<Vec<_>>(),
        YearsActive::ActiveInPast { periods } => periods,
    };

    periods.iter().map(|x| x.end - x.start).sum()
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
                SearchCondition::SearchByActiveYears { period } => {
                    was_artist_active(artist.to_owned(), period.to_owned())
                }
                SearchCondition::SearchByActiveLength { how_long, until } => {
                    active_length(artist.to_owned(), *until) >= *how_long
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
            years_active: YearsActive::StillActive {
                since: 1981,
                previous_periods: vec![],
            },
        },
        Artist {
            name: "Led Zeppelin",
            genre: MusicGenre::HardRock,
            origin: Location("England"),
            years_active: YearsActive::ActiveInPast {
                periods: vec![PeriodInYears {
                    start: 1968,
                    end: 1980,
                }],
            },
        },
        Artist {
            name: "Bee Gees",
            genre: MusicGenre::Pop,
            origin: Location("England"),
            years_active: YearsActive::ActiveInPast {
                periods: vec![PeriodInYears {
                    start: 1958,
                    end: 2003,
                }],
            },
        },
    ];

    // Zero condition
    let s0 = search_artists(artists.clone(), vec![]);

    for s in s0 {
        println!("{:?}", s);
    }

    // Search for pop artists from England active between 1950 and 2022
    let s1 = search_artists(
        artists.clone(),
        vec![
            SearchCondition::SearchByGenre {
                genres: vec![MusicGenre::Pop],
            },
            SearchCondition::SearchByActiveYears {
                period: PeriodInYears {
                    start: 1950,
                    end: 2022,
                },
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
                period: PeriodInYears {
                    start: 1950,
                    end: 2022,
                },
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
            period: PeriodInYears {
                start: 1950,
                end: 1979,
            },
        }],
    );
    for s in s1 {
        println!("{:?}", s);
    }

    // Search for artists active between 1981 and 1984
    let s1 = search_artists(
        artists.clone(),
        vec![SearchCondition::SearchByActiveYears {
            period: PeriodInYears {
                start: 1981,
                end: 1984,
            },
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
                period: PeriodInYears {
                    start: 2019,
                    end: 2022,
                },
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
                period: PeriodInYears {
                    start: 1950,
                    end: 1959,
                },
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
            period: PeriodInYears {
                start: 2019,
                end: 2022,
            },
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
            period: PeriodInYears {
                start: 1950,
                end: 2022,
            },
        },
    ];
    let s1 = search_artists(artists.clone(), search_conditions);
    for s in s1 {
        println!("{:?}", s);
    }

    let s1 = search_artists(
        artists.clone(),
        vec![SearchCondition::SearchByActiveLength {
            how_long: 40,
            until: 2023,
        }],
    );
    for s in s1 {
        println!("{:?}", s);
    }
}