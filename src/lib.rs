pub fn search<'a>(query: &str, contents: &'a str) -> impl Iterator<Item = &'a str> {
    contents.lines().filter(move |line| line.contains(query))
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> impl Iterator<Item = &'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(move |line| line.to_lowercase().contains(&query))
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
Pick three.
Duct tape.";
        let mut search_iter = search(query, contents);
        assert_eq!("safe, fast, productive.", search_iter.next().unwrap());
        assert_eq!(None, search_iter.next())
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Picke three.
Trust me.";
        let mut search_iter = search_case_insensitive(query, contents);

        assert_eq!("Rust:", search_iter.next().unwrap());
        assert_eq!("Trust me.", search_iter.next().unwrap());
        assert_eq!(None, search_iter.next());
    }
}
