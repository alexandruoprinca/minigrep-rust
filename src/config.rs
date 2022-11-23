pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        //first value is executable path, getting rid of it
        args.next();

        let query = match args.next() {
            Some(x) => x,
            None => return Err("Not enough arguments to get query"),
        };

        let file_path = match args.next() {
            Some(x) => x,
            None => return Err("Not enough arguments to get file path"),
        };

        let ignore_case = std::env::var("IGNORE_CASE_RST").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line: &&str| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query))
        .collect()
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(config.file_path)?;
    // println!("Found content: {}", contents);

    let myfunc = match config.ignore_case {
        true => search_case_insensitive,
        false => search,
    };

    for line in myfunc(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = r#"Rust:
safe, fast, productive.
Pick three."#;

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
        let query = "t";
        assert_eq!(
            vec!["Rust:", "safe, fast, productive.", "Pick three."],
            search(query, contents)
        );
    }
}
