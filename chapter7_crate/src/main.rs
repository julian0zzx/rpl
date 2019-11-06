
mod sound {
    pub fn guitar() {
        println!("guitar in sound");
    }

    pub mod voice {
        pub fn guitar() {
            // super::guitar(); // use super to call parent's mod/function
            println!("guitar in movie in sound");
        }
    }
}

fn main() {
    println!("Hello, world!");
    // crate is the ROOT of all mod
    crate::sound::guitar(); // guitar MUST be pub
    sound::voice::guitar(); // voice MUST be pub

    let a = Animal{name: String::from("Dog")};
    a.voice();
}

struct Animal {
    name : String
}
impl Animal {
    fn voice(&self) {
        println!("{}'s voice", self.name);
    }
}
