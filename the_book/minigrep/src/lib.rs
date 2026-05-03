// handles all the logic of the task at hand.

#![allow(unused)]

// lifetime parameters specify which argument lifetime is connected to the lifetime of the return value.
// In other words, we tell Rust that the data returned by the search function will live as long as the data passed into the search function in the contents argument.
// Other programming languages don’t require you to connect arguments to return values in the signature, but this practice will get easier over time.
//pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
pub fn search<'a>(query: &str, contents: &'a str) -> impl Iterator<Item = &'a str> {
    //unimplemented!();
    //vec![]

    //let mut results = Vec::new();
    //for line in contents.lines() {
    //    if line.contains(query) {
    //        results.push(line);
    //    }
    //}
    //results

    contents
        .lines()
        .filter(move |line| line.contains(query))
        //.collect()
}

//pub fn search_case_insensitive<'a>(
//    query: &str,
//    contents: &'a str,
//) -> Vec<&'a str> {
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> impl Iterator<Item = &'a str> {
    let query = query.to_lowercase();
    
    //let mut results = Vec::new();
    //for line in contents.lines() {
    //    if line.to_lowercase().contains(&query) {
    //        results.push(line);
    //    }
    //}
    //results

    contents
        .lines()
        .filter(move |line| line.to_lowercase().contains(&query))
        //.collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        //assert_eq!(vec!["safe, fast, productive."], search(query, contents));
        assert_eq!(vec!["safe, fast, productive."], search(query, contents).collect::<Vec<&str>>());
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        //assert_eq!(vec!["safe, fast, productive."], search(query, contents));
        assert_eq!(vec!["safe, fast, productive."], search(query, contents).collect::<Vec<&str>>());
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        //assert_eq!(
        //    vec!["Rust:", "Trust me."],
        //    search_case_insensitive(query, contents)
        //);
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents).collect::<Vec<&str>>()
        );
    }
}
