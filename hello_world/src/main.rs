#[derive(Debug)]
enum GradeLevel{
    Bachelor,
    Master,
    PhD,
}

#[derive(Debug)]
enum Major{
    ComputerScience,
    ElectricalEngineering,
}

#[derive(Debug)]
struct Student {
    name:String,
    grade:GradeLevel,
    major:Major,
}

impl Student{
    fn new(name: String, grade: GradeLevel, major: Major) -> Self{
        Student{
            name: name,
            grade: grade,
            major: major,
        }
    }

    fn introduce_yourself(&self){
        let name_msg = &self.name;

        let grade_msg = match self.grade{
            GradeLevel::Bachelor => "I am a Bachelor",
            GradeLevel::Master => "I am a Master",
            GradeLevel::PhD => "I have a PhD",
        };

        let major_msg = match self.major{
            Major::ComputerScience => "I major in Computer Science",
            Major::ElectricalEngineering => "I major in Electrical Engineering",
        };

        println!("My name is {}, {}, {}", name_msg, grade_msg, major_msg);
    }
}

fn main(){

    let s1:Student = Student::new("Tlaloc Alarcon".to_string(), GradeLevel::Bachelor, Major::ComputerScience);

    s1.introduce_yourself();
 
    //println!("{:?}", s1);
}