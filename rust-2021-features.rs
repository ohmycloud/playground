fn player_names(
    players: &[Player]
) -> impl Iterator<Item = &String> {
    players
        .iter()
        .map(|p| &p.name)
}

struct Player {
    name: String
}

trait Container {
    fn items(&self) -> impl Iterator<Item = &String>;
}

impl Container for Player {
    fn items(&self) -> impl Iterator<Item=&String> {
        todo!()
    }
}


fn main() {
    let players = [Player {name: "张三".into()}, Player {name: "张糸".into()}];
    let names = player_names(&players);
    for name in names {
        println!("{}", name);
    }
}
