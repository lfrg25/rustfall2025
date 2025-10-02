struct Student {
    name: String,
    major: String,
}


impl Student {
    fn new (s:String, m:String) -> Student {
        Student {
            name: s,
            major: m,
        }
    }

    fn get_major(&self) -> &String {
        &self.major
    }

    fn set_major(&mut self, new_major: String){
        self.major = new_major;  
    }
}

fn main() {
    let mut student = Student::new("Luis".to_string(), "Comp Sci".to_string());
    println! ("Student name: {} -  Major: {}", student.name, student.major);

    student.set_major("Video Editing".to_string());
    println!("New Major: {} ", student.get_major());
}