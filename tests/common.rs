pub enum Sex {
    Male,
    Female,
    Other,
}
impl Into<String> for Sex {
    fn into(self) -> String {
        match self {
            Sex::Male => "Male".into(),
            Sex::Female => "Female".into(),
            Sex::Other => "Other".into(),
        }
    }
}

pub struct Person {
    pub name: String,
    pub age: u8,
    pub sex: Sex,
}
