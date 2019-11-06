
pub fn slc_t() {
    println!("````````");
    
    let s0 = "hello world!"; // it's a immutable &str
    println!("{}", find_char(s0)); // will pass a str ref into find_char

    let s1 = String::from(s0);
    println!("{}", find_char_string(s1)); // pass value instead of ref
    // s1.clear(); // it's legal if a ref of string was passed into find_char_string
}

fn find_char(s : &str) -> usize {
    let btys = s.as_bytes();
    for (i, &itm) in btys.iter().enumerate() {
        if itm == b' ' {
            return i;
        }
    }
    return s.len();
}

fn find_char_String(s : String) -> usize {
    let btys = s.as_bytes();
    for (i, &itm) in btys.iter().enumerate() {
        if itm == b' ' {
            return i;
        }
    }
    return s.len();
}
