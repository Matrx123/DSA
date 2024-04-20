fn main() {
    println!("Frequency arrays");
}

fn frequency_array(arr: Vec<i32>) -> Vec<i32> {
    //size of this array will be largest element in input array + 1;
    let max = arr.iter().max().unwrap_or(&0);

    let mut freq = vec![0; (*max+1) as usize];
    for &item in &arr {
        freq[item as usize] += 1;
    }

    freq.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_frequency() {
        let result = frequency_array(vec![1, 2, 3, 1, 2, 4, 1, 3, 1, 5, 1]);
        assert_eq!(vec![0, 5, 2, 2, 1, 1], result);
    }
}
