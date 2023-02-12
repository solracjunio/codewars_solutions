fn main() {
}

#[allow(dead_code)]
fn string_to_array(s: &str) -> Vec<String> {
    let words: Vec<String> = s.split(" ").map(|s| s.to_string()).collect();
    return words;
}

#[cfg(test)]
mod tests {
    use super::string_to_array;

    fn do_test(s: &str, expected: &[&str]) {
        let actual = string_to_array(s);
        assert!(actual == expected, "Test failed with s = \"{}\"\nExpected {:?} but got {:?}", s, expected, actual);
    }

    #[test]
    fn fixed_tests() {
        do_test("Robin Singh", &["Robin", "Singh"]);
        do_test("CodeWars", &["CodeWars"]);
        do_test("I love arrays they are my favorite", &["I", "love", "arrays", "they", "are", "my", "favorite"]);
        do_test("1 2 3", &["1", "2", "3"]);
    }
}