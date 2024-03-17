#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice {
    A,
    B,
    C,
    D,
    E,
    F,
}
impl Choice {
    pub fn to_numeric(&self) -> u32 {
        match self {
            Choice::A => 1,
            Choice::B => 2,
            Choice::C => 3,
            Choice::D => 4,
            Choice::E => 5,
            Choice::F => 6,
        }
    }

    pub fn to_format(&self) -> String {
        match self {
            Choice::A => ".jpg".to_string(),
            Choice::B => ".png".to_string(),
            Choice::C => ".gif".to_string(),
            _ => "".to_string(),
        }
    }
}
