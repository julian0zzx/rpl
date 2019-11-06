
use std::cmp::PartialOrd;

fn main() {
    println!("Hello, trait!");

    let numbers = vec![2,5,8,1,4,9];
    println!("{:?}", largetst(&numbers));

    let chars = vec!['e', 'z', 'a', 'b', 'p', 'm'];
    println!("{:?}", largetst(&chars));

    // println!("{:?}", Point{x:23, y:12});
    // println!("{:?}", Point{x:2.3, y:1.2});
    println!("{:?}", Point{x:2.3, y:1.2});
    // println!("{:?}", Point::new(3, 8));
    println!("{:?}", Point::new(3, 8).x());
    
    println!("{:?}", Point::new(4, 5).size());
    
    measure(Point{x: 12, y: 15});

    // sizes();

    test_lifetime();
}

fn largetst<T: PartialOrd + Copy>(list : &[T]) -> T {
    let mut lgt = list[0];
    for &item in list.iter() {
        if item > lgt {
            lgt = item;
        }
    }
    return lgt;
} 

// trait
trait Size {
    // default implemtation
    fn size(&self) -> String {
        String::from("zero")
    }
}

// generalization
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T  // T means x and y must be with the same type, or use y: i32 to define y's type, or Point<T, U>
}

#[derive(Debug)]
struct Line<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
     pub const fn new(xx: T, yy: T) -> Self {
         return Self{x: xx, y: yy};
     }

     fn x(&self) -> &T {
         return &self.x;
     }
}

impl<T> Size for Point<T> {
    fn size(&self) -> String {
        String::from("0")
    }
}

impl<T> Size for Line<T> {
    fn size(&self) -> String {
        String::from("line-size")
    }
}

fn measure(obj: impl Size) {
    println!("obj measured, size: {}", obj.size())
}

// fn which_size(flag : bool) -> impl Size {
//     if flag {
//         Box::new(Point{x: 1, y: 2})
//     } else {
//         Box::new(Line{x: 1.5, y: 2.5})
//     }
// }

// lifetime
fn test_lifetime() {
    {
        let r;
        // println!("r:{}", r); // illegal, r is uninitialized
        {
            let x = 5;
            r = &x;
        }
        // println!("r:{}, x:{}", r, x); // out of x's scope
    }

    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is: {}", result);
}

// lifetime annotation 'a or 'x
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn lftm<'a>(x: &'a str, y: &'a str) -> &'a str {
    ""
}



