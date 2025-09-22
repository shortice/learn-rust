use std::{env, fs, process};

fn parse_cmd(args: &[String]) -> Result<(&str, &str), String> {
    if args.len() < 3 {
        return Err(
            String::from("Correct usage dagrep <query> <file>")
        )
    }

    let query = &args[1];
    let file_path = &args[2];

    Ok((query, file_path))
}

pub fn search<'a>(
    query: &str, 
    contents: &'a str,
    insensitive: bool
) -> Vec<&'a str> {
    let value = if insensitive {
        query.to_lowercase()
    } else {
        query.to_string()
    };

    contents.lines().filter(
        |x| x.to_lowercase().contains(value.as_str())
    ).collect()
}

fn main() -> Result<(), String> {
    let m: Vec<String> = env::args().collect();

    let (query, file_path) = parse_cmd(&m).unwrap_or_else(|err| {
        eprintln!("{err}");
        // Разница в том, что eprintln! печатает в не stdout
        // а в stderr.
        process::exit(0);
    });

    let content = fs::read_to_string(file_path)
        .expect("Read from path {file_path} is invalid.");

    let result = &search(&query, &content, true);

    if result.len() != 0 {
        for i in result {
            println!("{}", i);
        }
    } else {
        println!("Nothing here.");
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."], 
            search(query, contents, false)
        );
    }

    #[test]
    fn two_result() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search(query, contents, true)
        );
    }
}

