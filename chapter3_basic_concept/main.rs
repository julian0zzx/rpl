//! ### mutable variable
//! ### help doc
//! * wow
//! * lol

const UNIQ_VALUE : u32 = 127;
// const UNIQ_VALUE : u32 = 122; // redefined
fn main() {
    let mut x = 5; // mutable
    println!("the value of x is: {}", x);
    x = 6; 
    println!("the value of x is: {}, UNIQ_VALUE is: {}", x, UNIQ_VALUE);
    
    let y = 1; // unused, from compiler view point
    let y = y + 2; // immutable, but shadowing, using y + 2 to remove the warning, unused y
    println!("shadowed y with new value: {}", y);

    let magic = "this is a string";
    let magic = magic.len(); // magic, shadowing convert data value and the data type at the same time
    println!("magic is converted from string to uint: {}", magic);

    println!("{}", fn_length("magic is converted from string to uint"));
    fn_tuple();
    fn_expr();

    // let ary = [1, 2, 3 , 5];
    // println!("{}", ary);

    fn_branches();

    fn_loop_while_for();
}

fn fn_loop_while_for() {
    let mut x = 1;
    let a = loop {
        x += 1;
        if x > 3 {
            break x; // return x
        }
    };
    println!("{}", a);

    while x < 8 {
        x += 1;
    }
    println!("{}", x);

    let ary = [1, 3, 4, 6, 8];
    for a in ary.iter() { // like foreach in Java
        print!("{} ", a);
    }
    
    let mut i = 0; 
    while i < ary.len() / 2 {
        print!("{} ", ary[i]);
        i += 1;
    }

    let r = 1..10; // range from 1 to 9, 
    for i in r {
        print!("{} ", i);
    }
    let r1 = 1..5; // r was moved
    let rr : Vec<_> = r1.map(|x| x * 2).collect();
    for i in rr {
        print!("{} ", i);
    }
}

fn fn_branches() {
    let n = 3;
    let n = if n < 5 {
        n + 1
    } else {
        n - 1
    };
    if n < 5 {
        println!("less than 5");
    } else if n == 5 {
        println!("equals to 5")
    } else {
        println!("greater than 5");
    };
}

fn fn_expr() {
    let x = 5;
    let y = {
        let x = 4; // shadowing
        x + 1 // if it ends with ; then it will be a statement not a experssion
    };
    println!("{}", y);
}

fn fn_length(s : &str) -> usize {
    return s.len();
}

fn fn_tuple() {
    let tup3 = (200, "OK", "looks good");
    // println!("tuple: {}", tup3);
    println!("tuple[2]: {}", tup3.2);
    let (_, status, _desc) = tup3; // use _ to omit , use to _desc to ignore warning msg if not used
    println!("{}", status);
}

// impl std::fmt::Display for (i32, &str, &str) {
//     fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "tup3(i32, &str, &str): ({}, {}, {})", self.0, self.1, self.2)
//     }
// }
