use std::collections::HashMap;

/// https://twitter.com/oliviff/status/1482720342540009474
fn main() {
    // Keep track of professors name ->  list of courses
    let mut course_database = CourseDatabase::new();

    // New course for John Smith
    let john_smith_security = Professor::new("John Smith", 1990);
    let john_smith_biology = Professor::new("John Smith", 2022);

    let cs_101 = Course::new("CS_101");
    let security_101 = Course::new("Security 101");
    let biology_101 = Course::new("Biology 101");


    course_database.add_course(john_smith_security.clone(), cs_101);
    course_database.add_course(john_smith_security, security_101);
    course_database.add_course(john_smith_biology, biology_101);

    println!("{:#?}", course_database);
}

/// Imagine we are a university and we want to keep track of our professors and the courses they teach.
/// Let's implement a simple in-memory database using a HashMap and discuss various HashMap operations.

#[derive(Debug, PartialEq)]
struct Course {
    name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Professor {
    name: String,
    year_of_birth: u32, // Perhaps id number?
}

#[derive(Debug)]
struct CourseDatabase {
    // Maps Professor -> list of courses they teach
    database: HashMap<Professor, Vec<Course>>,
}

impl Course {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl Professor {
    pub fn new(name: &str, year_of_birth: u32) -> Self {
        Self {
            name: name.to_string(),
            year_of_birth,
        }
    }
}

impl CourseDatabase {
    pub fn new() -> Self {
        Self {
            database: HashMap::new(),
        }
    }

    /// We check if the key is already there,
    /// if it is we update it, otherwise we insert a new key with our new value.
    // fn add_course2(&mut self, professor: &Professor, course: Course) {
    //     // Check if this professor exists in the database
    //     if self.database.contains(&professor.name) {
    //         // Update the existing vector
    //         if let Some(courses) = self.get_mut(&professor.name) {
    //             courses.push(course);
    //         } else {
    //             // Insert a new value with the new course
    //             self.database.insert(professor.name.clone(), vec![course]);
    //         }
    //     }
    // }

    /// The above code works, but it's quite verbose. Let's explore Rust's entry API to simplify.
    pub fn add_course(&mut self, professor: Professor, course: Course) {
        self.database
            .entry(professor)
            .or_insert(Vec::new()) // &mut Vec<Course>
            .push(course);
    }

    ///We use the entry API again,
    /// and Vec::retain to keep all courses that don't match the course we are trying to delete.
    pub fn remove_course(&mut self, professor: Professor, course: Course) {
        self.database
            .entry(professor)
            .or_insert(Vec::new())
            .retain(|c| c.name != course.name);
    }

    /// compile error
    // pub fn is_teaching(&self, professor: Professor, course: Course) -> bool {
    //     self.database
    //     .entry(professor)
    //     .or_insert(Vec::new())
    //     .contains(&course)
    // }

    /// So in this case perhaps the entry API is not what we need,
    /// because we don't need mutable access to the value.
    /// Let's use get instead, which gives us a non-mutable reference.
    pub fn is_teaching(&self, professor: &Professor, course: Course) -> bool {
        self
            .database
            .get(&professor) // Optioin<&Vec<Course>>
            .map(|courses| courses.contains(&course)) // Option<bool>
            .unwrap_or_default() // bool value from above or false if None
    }

    // Using professors as key

    // Our university is very popular. We have just hired another John Smith,
    // who is an expert in Biology. But sadly our database implementation won't allow us to
    // have two separate John Smiths, as we use the professor name as the HashMap key.
}
