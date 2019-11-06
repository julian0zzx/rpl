
fn main() {
    println!("hello struct");
    let u1 = User {
        name: String::from("Tom"),
        email: String::from("tom@gmail.com"),
        age: 24,
        active: true
    };

    let u2 = User::build_user(String::from("Jerry"), String::from("jerry@gmail.com"), 16);
    let u3 = User {
        name : String::from("Bob"),
        email: String::from("bob@gmail.com"),
        ..u2 // other properties coming from u2
    };

    let origin_point = Color(0,0,0);

    println!("{:?}, it's {} years old in next year, is it greater than 12: {}", u3, u3.next_year_age(), u3.greater_than(12)); // :? print debug info of User, :#? 
}
struct Color(i32, i32, i32);

#[derive(Debug)] // to enable debug info, :? and :#?
struct User {
    name: String, // if its type was str, then got the error: doesn't have a size known at compile-tim
    email: String, // , instead of ;
    age: u16,
    active: bool
}

impl User {
    // function not method, 
    fn build_user(name : String, email : String, age : u16) -> User {
        User{
            name, email, age, active: true // param name was mapping to name!
        }
    }
    // method of User, not function
    // &self
    fn next_year_age(&self) -> u16 {
        self.age + 1
    }
    // first parament can be omitted when calling
    fn greater_than(&self, age : u16) -> bool {
        self.age > age
    }
}
