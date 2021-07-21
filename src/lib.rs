use std::fs;
use std::error::Error;
use std::env;


pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    //? Implementing new function allows us to create instances of Config via Config::new
    pub fn new(args: &[String]) -> Result<Config, &str> {
        // Note: args[0] contains 'target/debug/minigrep'
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        // Note: clone is not necessarily the optimal solution in terms of performance. However, 
        //       it solves an ownership issue with the String objects passed to the Config struct. Given
        //       the simplicity of this program, we can acceptably make this concession
        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { 
            query, 
            filename, 
            case_sensitive 
        })
    }
}

// Note: Box<dyn Error> ensures that the function returns a value that implements the Error trait
//      if the function fails. dyn is short for 'dynamic'
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //? the ? operator means an error will be returned on an error
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    //? Returning () is the idiomatic way to say that we're calling run for the side effects
    //? only (i.e. Error handling) and that it will not return a value
    Ok(())
}

// Note: assigning liftime 'a to contents param to ensure that the reference to the slice in contents will
//      live as long as the value returned in results
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

// ########## TESTS ########## //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    #[should_panic]
    fn check_num_args() {
        let args:Vec<String> = vec![String::from("arg1")];

        let config = Config::new(&args).expect("Not enough args");
    }

    #[test]
    #[should_panic]
    fn run_err_check() {
        let args:Vec<String> = vec![String::from("/some/file"), String::from("needle"), String::from("haystack.txt")];

        let test_config = Config::new(&args).unwrap();

        run(test_config).expect("Err")
    }

    #[test]
    fn new_config_test() {
        let args:Vec<String> = vec![String::from("/some/file"), String::from("needle"), String::from("haystack")];

        let test_config = Config::new(&args);

        assert!(test_config.is_ok())
    }
}