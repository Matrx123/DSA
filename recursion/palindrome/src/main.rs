fn main() {
    println!("Check if a string is a palindrome");

    let pal = "MALAYALAM";
    let modified_str: Vec<char> = pal.chars().collect();
    let is_palindrome = check_if_palindrome(&modified_str, 0 as usize, modified_str.len());

    println!("String {:?} is Palindrome :: {:?}", pal, is_palindrome);
}

fn check_if_palindrome(pal: &Vec<char>, i: usize, j: usize) -> bool {
    if i > j { true } else { pal[i] == pal[j - 1] && check_if_palindrome(pal, i + 1, j - 1) }
}
