use std::collections::HashMap;

///https://twitter.com/oliviff/status/1485265274618142725
fn main() {
    let student_bob = Student::new("Bob");
    let student_sue = Student::new("Sue");

    let professor_john = Professor::new("John");

    let mut database = Database {
        students: HashMap::from([(StudentId(1), student_bob), (StudentId(2), student_sue)]),
        professors: HashMap::from([(ProfessorId(1), professor_john)]),
        course_registrations: Vec::new(),
    };

    // Register course
    database.add_course_registration(ProfessorId(1), StudentId(1)); // John teaches Bob
    database.add_course_registration(ProfessorId(1), StudentId(2)); // John teaches Sue

    let john: ProfessorId = ProfessorId(1);
    let bob: StudentId = StudentId(1);
    let sue: StudentId = StudentId(2);

    // shouble be true
    println!(
        "Bob is registered for John's course: {}",
        database.is_registered(john.clone(), bob)
    );

    println!(
        "Sue is registered for John's course: {}",
        database.is_registered(john, sue)
    );
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProfessorId(u32);
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StudentId(u32);

pub struct Student {
    name: String,
}

pub struct Professor {
    name: String,
}

impl Student {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl Professor {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

pub struct Database {
    // professor_id -> professor map
    professors: HashMap<ProfessorId, Professor>,
    // student_id -> student map
    students: HashMap<StudentId, Student>,
    // professor_id -> student_id
    course_registrations: Vec<(ProfessorId, StudentId)>,
}

impl Database {
    pub fn get_professsor(&self, professor_id: ProfessorId) -> Option<&Professor> {
        self.professors.get(&professor_id)
    }
    pub fn get_student(&self, student_id: StudentId) -> Option<&Student> {
        self.students.get(&student_id)
    }
    pub fn add_course_registration(&mut self, professor_id: ProfessorId, student_id: StudentId) {
        self.course_registrations.push((professor_id, student_id));
    }
    pub fn is_registered(&self, professor_id: ProfessorId, student_id: StudentId) -> bool {
        self.course_registrations
            .contains(&(professor_id, student_id))
    }
}
