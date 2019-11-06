use std::collections::HashMap;

fn main() {
    vec_test();
    string_test();
    string_index_test();
    map_test();
}

fn vec_test() {
    // <Generics>
    let mut v1: Vec<i32> = Vec::new();
    v1.push(12);
    v1.push(32);
    dbg!(v1.pop());

    // Rust check vv's type <Generics>
    let mut vv = Vec::new();
    vv.push(18);
    vv.push(23);
    dbg!(vv.pop());

    let v2 = vec![2, 3, 5];
    dbg!(v2[2]); // v2.pop() will change the Vector
    dbg!(v2.get(1)); // index from 0, out of bound index will return None
    dbg!(v2.get(1).unwrap()); // index from 0

    let mut v3 = vec![1, 2, 3];
    // let first = &v3[0];
    v3.push(5);
    // let last = &v3.pop();
    dbg!(v3.len());

    for i in v2 {
        dbg!(i);
    }
}

fn string_test() {
    let mut s1 = String::new();
    s1.push_str("Hello");
    s1.push_str("_");
    s1.push_str("World!");
    dbg!(s1);
    // s1.push_str(s0); // s1 moved

    let s2 = "hello String".to_string();
    dbg!(s2);

    let s3 = String::from("ha ha ha hahaha");
    dbg!(s3);

    // str can be +, use &s3 instead for String, &String coerced to a &str
    // let s4 = s2 + &s3; // s2&s3 moved
    let s11 = String::from("hehaha");
    let s22 = String::from("woorld");
    // let s00 = "woooo";
    // let s44 = s11 + s00; // + str  or + &String
    let s44 = s11 + &s22;
    // let s44 = format!("{}-{}", s11, s22);
    dbg!(s44);
}

fn string_index_test() {
    let s1 = String::from("hello你好");
    dbg!(s1.len()); // 5+6=11， utf8 3 bytes
    let s2 = "world世界"; 
    dbg!(&s2[0..8]); // range will step into utf8 bytes, only special range will be accepted
    for b in "wo我".bytes() {
        dbg!(b);
    }
    for c in "wo我".chars() {
        dbg!(c);
    }
}

fn map_test(){
    let mut scores = HashMap::new();
    scores.insert("ten", 10);
    scores.insert("twenty", 20);
    dbg!(scores.get("ten"));

    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores2 : HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    dbg!(scores2.keys());

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut mappp = HashMap::new();
    mappp.insert(field_name, field_value);
    for (k,v) in mappp {
        dbg!(k);
        dbg!(v);
    }
    
    dbg!(scores.get("ten"));
    scores.insert("ten", 100);
    dbg!(scores.get("ten"));
    scores.entry("ten*100").or_insert(1000);
    dbg!(scores.get("ten*100"));
    
}
