
use std::env;
use std::process;
use minigrep;
use minigrep::Config;

fn main() {
    println!("Hello, IO!");

    let args : Vec<String> = env::args().collect(); // 0 command, 1 first arg, 2 second arg


    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("parsing args: {:?}", err);
        process::exit(1);
    });

    // let content = fs::read_to_string(filename).expect("file not exists");

    // println!("{:?}", content); // 0 command, 1 first arg, 2 second arg

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {:?}", e);
        process::exit(1);
    }

}

// // fn parse_cmd(args : &[String]) -> (String, String) {
// //     return (args[1].clone(), args[2].clone());
// // }

// fn parse_cmd(args : &[String]) -> (&str, &str) {
//     if args.len() < 3 {
//         panic!("no enough args");
//     }
//     return (&args[1], &args[2]);
// }



