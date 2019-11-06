
use std::fs;
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    query : String,
    filename : String
}

impl Config {
    pub fn new(args : &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("no enough args");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        // return
        Ok(Config{query, filename})
    }
}

pub fn run(cfg : Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(cfg.filename).expect("file not exists");

    println!("{:?}", content); // 0 command, 1 first arg, 2 second arg
    for line in search(&cfg.query, &content) {
    // for line in search2(&cfg.query, &content) {
        println!("found: {:?}", line);
    }

    return Ok(());
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut resutls = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            resutls.push(line.trim())
        }
    }
    
    resutls
}

// using iterator
fn search2<'a>(query : &str, contents : &'a str) -> Vec<&'a str> {
    return contents.lines().filter(|line| line.contains(query)).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "safe";
        let content = "
        Rust:
        safe, fast, productive.
        Pick three.
        rust query test";
        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

}


