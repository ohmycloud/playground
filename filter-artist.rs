#[derive(Debug)]
pub struct User {
    name: String,
    city: Option<String>,
    favorite_artists: Vec<String>
}

// users that haven't specified their city or live in Melbourne
pub fn f1(users: Vec<User>) -> Vec<User> {
    users
        .into_iter()
        .filter(|x| 
            x.city.is_none() || 
            x.city.as_ref().is_some_and(|city| city == "Melbourne")
        )
        .collect()
}

// users that live in Lagos
pub fn f2(users: Vec<User>) -> Vec<User> {
    users
        .into_iter()
        .filter(|x| x.city.as_ref().is_some_and(|city| city == "Melbourne"))
        .collect()
}

// users that like Bee Gees
pub fn f3(users: Vec<User>) -> Vec<User> {
    users
        .into_iter()
        .filter(|x| x.favorite_artists.contains(&"Bee Gees".to_string()))
        .collect()
}

// users that live in cities that start with the letter T
pub fn f4(users: Vec<User>) -> Vec<User> {
    users
        .into_iter()
        .filter(|x| x.city.as_ref().is_some_and(|city| city.starts_with("L")))
        .collect()
}

// users that only like artists that have a name longer than eight characters(or no favorite artists at all)
pub fn f5(users: Vec<User>) -> Vec<User> {
    users
        .into_iter()
        .filter(|x| x.favorite_artists.clone().into_iter().all(|x| x.len() > 8))
        .collect()
}

// users that like some artists whose names start with an M
pub fn f6(users: Vec<User>) -> Vec<User> {
    users
        .into_iter()
        .filter(|x| x.favorite_artists.clone().into_iter().any(|x| x.starts_with("M")))
        .collect()
}

fn main() {
    let users = vec![
        User {name: "Alice".to_string(),   city: Some("Melbourne".to_string()),    favorite_artists: vec!["Bee Gees".to_string()]},
        User {name: "Bob".to_string(),     city: Some("Lagos".to_string()),        favorite_artists: vec!["Bee Gees".to_string()]},
        User {name: "Eve".to_string(),     city: Some("Tokyo".to_string()),        favorite_artists: vec![]},
        User {name: "Mallory".to_string(), city: None,                             favorite_artists: vec!["Metallica".to_string(), "Bee Gees".to_string()]},
        User {name: "Trent".to_string(),   city: Some("Buenos Aires".to_string()), favorite_artists: vec!["Led Zeppelin".to_string()]}
    ];

    // Your task is to implement six functions that take a list of users
    // that satisfy a given condition.
    
    for city in f6(users) {
        println!("{:?}", city);
    }
}