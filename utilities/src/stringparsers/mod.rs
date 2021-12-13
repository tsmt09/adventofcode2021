pub fn parse_string_as_vec_int<T: std::str::FromStr + std::fmt::Display>(
    line: &String,
    seperator: char,
) -> Vec<T> {
    line.split(seperator)
        .map(|e| match e.parse::<T>() {
            Ok(e) => e,
            Err(_) => panic! {"Error parsing {}", e},
        })
        .collect()
}
