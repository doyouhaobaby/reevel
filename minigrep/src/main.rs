use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    //let mut args: Vec<String> = env::args().collect();
    //args.push("hello world".to_string());

    //let (query, filename) = parse_config(&args);
    //let config = parse_config(&args);
    //let config = Config::new(&args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // let query = &args[1];
    // let filename = &args[2];

    // //println!("{:?}", args);
    // println!("Search for {}", query);
    // println!("In file {}", filename);
    println!("Search for {}", config.query);
    println!("In file {}", config.filename);

    // let contents = fs::read_to_string(filename)
    //     .expect("Something went wrong reading the file");
    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");
    
    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

// fn parse_config(args: &[String]) ->(&str, &str) {
//     let query = &args[1];
//     let filename = &args[2];

//     (query, filename) 
// }

// fn parse_config(args: &[String]) -> Config {
//     // let query = args[1];
//     // let filename = args[];
//     let query = args[1].clone();
//     let filename = args[2].clone();

//     Config {query, filename}
// }

// impl Config {
//     fn new(args: &[String]) -> Config {
//         if args.len() < 3 {
//             panic!("not enough arguments");
//         }

//         // let query = args[1];
//         // let filename = args[];
//         let query = args[1].clone();
//         let filename = args[2].clone();

//         Config {query, filename}
//     } 
// }
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // let query = args[1];
        // let filename = args[];
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {query, filename})
    } 
}
