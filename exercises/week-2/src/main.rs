use std::fmt::Display;

fn to_letter_grade(num: u8) -> String {
    match num {
        0..=59 => "FF".to_string(),
        60..=69 => "DD".to_string(),
        70..=79 => "CC".to_string(),
        80..=89 => "BB".to_string(),
        90..=100 => "AA".to_string(),
        _ => "Out of Scope".to_string(),
    }
}

enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

fn log(level: LogLevel, msg: &str) {
    match level {
        LogLevel::Debug => println!("[DEBUG]: {}", msg),
        LogLevel::Info => println!("[INFO]: {}", msg),
        LogLevel::Warn => println!("[WARN]: {}", msg),
        LogLevel::Error => println!("[ERROR]: {}", msg),
    }
}

enum Gender {
    Male,
    Female,
}

struct Person {
    name: String,
    age: u8,
    gender: Gender,
}

impl Display for Gender {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match *self {
            Gender::Male => write!(f, "Male"),
            Gender::Female => write!(f, "Female"),
        }
    }
}

impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} , {} , {}", self.name, self.age, self.gender)
    }
}


macro_rules! display {
    ($struct_name:ty, $format_string:literal, $($fields:ident),+ $(,)?) => {
        impl Display for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $format_string, $( self.$fields ),*)
            }
        }
    };
}
display!(Person, "{}, {}, {}", name, age, gender);

fn main() {
    println!("{}", to_letter_grade(90));
    log(LogLevel::Debug, "This is a debug message");
    log(LogLevel::Info, "This is an info message");
    log(LogLevel::Warn, "This is a warn message");
    log(LogLevel::Error, "This is an error message");

    let person1 = Person {
        name: String::from("arda"),
        age: 18,
        gender: Gender::Male,
    };

    let person2 = Person {
        name: String::from("hello"),
        age: 22,
        gender: Gender::Female,
    };

    println!("{person1}");
    println!("{person2}");
}
