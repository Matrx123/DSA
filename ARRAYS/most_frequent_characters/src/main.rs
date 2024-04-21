use std::collections::HashMap;

fn main() {
    println!("Most occuring character");
}

fn most_occuring(text: &str) -> Option<char> {
    let freq = text.chars();
    let mut char_to_occurence = HashMap::new();
    let mut max = 0;
    let mut res: Option<char> = None;
    for item in freq {
        *char_to_occurence.entry(item).or_insert(0) += 1;
    }
    for (key, value) in char_to_occurence {
        if value > max {
            max = value;
            res = Some(key);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_most_occuring() {
        assert_eq!(Some('l'), most_occuring("hello"));
        assert_eq!(Some(' '), most_occuring("a b c d e f g h i j k"));
        assert_eq!(None, most_occuring(""));
        assert_eq!(Some('a'), most_occuring("aaaa"));
    }
}
