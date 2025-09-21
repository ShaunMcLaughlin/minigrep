pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub include_file: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // separate the args into final args and options
        let mut finalargs = vec![];
        let mut options = vec![];

        // iterate over all args
        for i in 0..args.len() {
            let arg = args[i].clone();
            // options start with - and may be multiple so separate into chars and
            // add to list of options
            if &arg[0..1] == "-" {
                let option : Vec::<char>= arg[1..].chars().collect();
                for opt in option {
                    options.push(opt);
                }
            } else {
                // otherwise add arg to final args
                finalargs.push(arg);
            }
        }

        // create config variables
        let ignore_case = options.contains(&'i');
        let include_file = options.contains(&'f');
        let file_path = finalargs.remove(2);
        let query = finalargs.remove(1);

        Ok(Config { query, file_path, ignore_case, include_file })
    }
}

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
        )
    }

}
