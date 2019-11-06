
fn main() {
    println!("Hello, pattern!");

    // 18.1
    // test_match();
    // test_if();
    // test_while();
    // test_for();
    // test_let();
    // test_fn();
    
    // 18.2
    // test_refutable();

    // 18.3
    test_syntax();

}

// 18.1
fn test_match() {
    let ss = "H";
    match ss {
        "H" => println!("really? {}", ss),
        _ => println!("NONE")
    }
}

fn test_if() {
    let mut fav_color : Option<&str> = None;
    let is_tuesday  = false;
    let age : Result<u8, _>  = "34".parse();

    // fav_color = Some("red");
    if let Some(color) = fav_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

}

fn test_while() {
    // let mut stock = Vec::new();
    let mut stock = vec![];
    stock.push(1);
    stock.push(2);
    stock.push(3);
    while let Some(top) = stock.pop() { // pop one by one
        println!("top: {}", top);
    }
}

fn test_for() {
    let vv = vec!["one", "two", "three"];
    for (idx, v) in vv.iter().enumerate() {
        println!("{} indexed with {}", v, idx);
    }
}

fn test_let() {
    let (x, y, z) = (1, 4, 6);
    println!("x {}, y {}, z {}", x, y, z);
    let (x, y, z) = (2, 7, 13);
    println!("x {}, y {}, z {}", x, y, z);
    let (x, y, _) = (3, 9, 10);
    println!("x {}, y {}, z {}", x, y, z); // z = 13

}
fn test_fn() {
    let (px, py) = (10, 33);
    test_fnnn(&(px, py));
}
fn test_fnnn(&(x, y) : &(i32, i32)) {
    println!("point is ({}, {})", x, y);
}

// 18.2
fn test_refutable() {
    // let oo : Option<&str> = None;
    // let Some(a) = oo; // pattern 'None' not covered
    if let x = 4 {
        println!("lol");
    }
}

// 18.3
fn test_syntax() {
    let x = 4;
    match x {
        1 | 3 => println!("one, three. odd"),
        2 => println!("two"),
        4 ... 8 => println!("4/5/6/7/8"), // "..." but ".." in for
        _ => println!("any")
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        // Some(z) => println!("Matched, z = {:?}", z),
        _ => println!("Default, x = {:?}", x),
    }

    let point  = Point{x : 1.0, y : 3.3};
    let Point{x: a, y: b} = point;
    println!("x: {}, y: {}", a, b);
    let Point{x , y} = point;
    println!("x: {}, y: {}", x, y);

    let point00 = Point{x: 0.0, y: 11 as f64};
    match point00 {
        Point{x: 0f64, y} => println!("on y axis, y is {}", y),
        Point{x, y: 0f64} => println!("on x axis, x is {}", x),
        Point{x, y} => println!("on neither axis, (x,y) is ({},{})", x, y),
    }

    // let msg = Msg::ChangeColor(10, 90, 180);
    let msg = Msg::Move{x : 90, y : 180};
    match msg {
        Msg::Quit => println!("quitting ___"),
        Msg::Move{x, y} => println!("move to (x, y) ({},{})", x, y),
        Msg::Write(text) => println!("Text message: {}", text),
        Msg::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }

    let points = vec![Point{x : 1.0, y : 2.2}, Point{x : 2.3, y : 8.5}, Point{x : 4.3, y : 7.5}];
    let sum_of_squares : f64 = points.iter().map(|Point{x, y}| x*x + y*y).sum(); // both Point and &Point are OK
    println!("sos {}", sum_of_squares);
    foo(3, 5);

    let s = Some(String::from("Hello!"));
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);

    let trr = (1, 2, 4, 9);
    match trr {
        (1, ..) => println!("Starts with 1"),
        _ => println!("Starts don't with 1"),
    }
    let trr = (1, 2, 4, 9);
    match trr {
        (ff, .., ll) => println!("Starts with {}, ends with {}", ff, ll),
        _ => println!("Starts don't with 1"),
    }

    let sm3 = Some(30);
    match sm3 {
        Some(x) if x < 5 => println!("{} is less than 5", x),
        Some(y) => println!("{} is not less than 5", y),
        _ => println!("nothing"), // can't remove this line
    }

    let x = 7;
    let fflg = true;
    match x {
        1 | 3 | 5 if fflg => println!("odd"),
        a @ 7 => println!("{} equals to 7!", a),
        x @ 10 ... 20 => println!("{} between 10 ... 20", x),
        _ => println!("none"),
    }

    let robot_name = &Some(String::from("Bors"));
    match robot_name {
        Some(name) => println!("Found a name: {}", name), // same as &Some(ref name)
        None => (),
    }
    println!("robot_name is: {:?}", robot_name);
}

fn foo(_ : i32, y : i32) { // magic! Totally omit first parameter
    println!("{}", y);
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
enum Msg {
    Quit, 
    Move {x : i32, y : i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

