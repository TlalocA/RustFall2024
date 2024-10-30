#[derive(Debug, PartialEq)]

pub enum Major {
    CompSci,
    ElecEng,
    Undefined,
}

impl Major{
    pub fn classify(major:&str) -> Self{
        match major{
            "CS" => Major::CompSci,
            "EE" => Major::ElecEng,
            _ => Major::Undefined,
        }
    }
}