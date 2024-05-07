use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn get_anagram(txt: &str, pat: &str) -> i32 {
    let mut i = 0;
    let mut j = 0;
    let mut pat_freq = HashMap::new();

    let mut result = 0;
    let arr: Vec<char> = txt.chars().collect();
    let n = arr.len();
    let pat_arr: Vec<char> = pat.chars().collect();
    let k = pat_arr.len();

    for item in pat_arr {
        *pat_freq.entry(item).or_insert(0) += 1;
    }
    let mut count = pat_freq.len();

    while j < n {
        if let Some(freq) = pat_freq.get_mut(&arr[j]) {
            *freq -= 1;
            if *freq == 0 {
                count -= 1;
            }
        }

        if j - i + 1 == k {
            if count == 0 {
                result += 1;
            }
            if let Some(freq) = pat_freq.get_mut(&arr[i]) {
                *freq += 1;
                if *freq == 1 {
                    count += 1;
                }
            }
            i += 1;
            j += 1;
        } else {
            j += 1;
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_anagram() {
        let txt = "forxxorfxdofr";
        let pat = "for";
        assert_eq!(3, get_anagram(txt, pat));
        let txt = "ababaaababaa";
        let pat = "aba";
        assert_eq!(7, get_anagram(txt, pat));
        let txt = "kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk";
        let pat = "kkkkk";
        assert_eq!(37, get_anagram(txt, pat));
    }
}
