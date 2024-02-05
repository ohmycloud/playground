#[derive(Debug)]
struct Student<'a> {
    name: &'a str,
    score: u32,
    grade: char,
}

fn main() {
    let names = vec!["Alice", "Bob", "Charlie"];
    let scores = vec![90, 85, 70];
    let grades = vec!['A', 'B', 'C'];

    let students = names.iter()
        .zip(scores.iter())
        .zip(grades.iter())
        .map(|((name, score), grade)| Student { name, score: *score, grade: *grade })
        .collect::<Vec<_>>();


    for student in students {
        println!("{:?}", student);
    }
}
