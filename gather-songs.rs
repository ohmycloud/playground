#[warn(dead_code)]

use std::collections::HashSet;

// model a playlist
// 1. Playlist has a name, a kind, and a list of songs.
// 2. There are three kinds of playlists: curated by a user, based on a particular artist,
//    and based on a specific set of genres.
// 3. A song has an artist and a name.
// 4. A user has a name.
// 5. An artist has a name.
// 6. There are only three music genres: use your three favorite genres.
pub struct User<'a>(&'a str);
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Artist<'a>(&'a str);
#[derive(Clone, Debug)]
pub struct Song<'a> {
    title: &'a str,
    artist: Artist<'a>
}

#[derive(Clone)]
pub struct Playlist<'a> {
    name: &'a str,
    kind: &'a PlaylistKind<'a>,
    songs: Vec<Song<'a>>
}

#[derive(Eq, Hash, PartialEq)]
pub enum MusicGenre {
    House,
    Funk,
    HipHop
}

pub enum PlaylistKind<'a> {
    CuratedByUser {user: User<'a>},
    BasedOnArtist {byartist: Artist<'a>},
    BasedOnGenres {genres: HashSet<MusicGenre>}
}

macro_rules! hashset {
    ($($key: expr),*) => {{
         let mut set = ::std::collections::HashSet::new();
         $( set.insert($key); )*
         set
    }}
}

// It should return some songs from given playlists,
// namely, songs performed by the artist from user-based playlists
// plus all songs from artist-based playlists
// plus all songs from genre-based playlists
// 合并用户提供的播放列表, 基于歌手的播放列表, 基于流派的播放列表
pub fn gather_songs<'a>(playlists: Vec<Playlist<'a>>, artist: Artist, genre: MusicGenre) -> Vec<Song<'a>> {
    playlists.iter()
            .fold(vec![], |mut songs, playlist| {
                let mut matching_songs = match playlist.kind {
                    PlaylistKind::CuratedByUser {user}   => playlist.songs.clone().into_iter().filter(|x| x.artist == artist).collect::<Vec<Song>>(),
                    PlaylistKind::BasedOnArtist {byartist} => if *byartist == artist { playlist.songs.clone() } else { vec![] },
                    PlaylistKind::BasedOnGenres {genres} => if genres.contains(&genre) { playlist.songs.clone() } else { vec![] }
                };
                songs.append(&mut matching_songs);
                songs
            })
}

fn main() {
    let pk1 = &PlaylistKind::BasedOnArtist {byartist: Artist("Foo Fighters")};
    let pk2 = &PlaylistKind::BasedOnGenres {genres: hashset![MusicGenre::House, MusicGenre::Funk]};
    let pk3 = &PlaylistKind::CuratedByUser  {user: User("ohmycloud")};

    let playlist = vec![
        Playlist {name: "This is Foo Fighters", kind: pk1, songs: vec![Song {title: "Breakout",      artist: Artist("Foo Fighters")}, Song {title: "Learn To Fly",     artist: Artist("Foo Fighters")}]},
        Playlist {name: "Deep Focus",           kind: pk2, songs: vec![Song {title: "One More Time", artist: Artist("Daft Punk")},    Song {title: "Hey Boy Hey Girl", artist: Artist("The Chemical Brothers")}]},
        Playlist {name: "ohmycloudy",           kind: pk3, songs: vec![Song {title: "Only Time",     artist: Artist("Enya")},         Song {title: "你",               artist: Artist("Gala")}]}
    ];

    println!("{:?}", gather_songs(playlist.clone(), Artist("Foo Fighters"), MusicGenre::House));
    println!("{:?}", gather_songs(playlist, Artist("Enya"), MusicGenre::HipHop));
}