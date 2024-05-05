fn main() {
    println!("LeetCode problem no : 974. Sub_array Sums Divisible by K");
}

fn get_subarray(data: Vec<i32>, k: i32) -> i32 {
    let mut prefix_sum_array = Vec::new();
    let mut sum = 0;
    let _: Vec<i32> = data
        .iter()
        .map(|&item| {
            sum += item;
            prefix_sum_array.push(sum);
            item
        })
        .collect();
    let mut modulo_array = Vec::new();
    let _: Vec<i32> = prefix_sum_array
        .iter()
        .map(|&item| {
            modulo_array.push(((item % k) + k) % k);
            item
        })
        .collect();

    let mut freq_array = vec![0; k as usize];
    freq_array[0] = 1;
    let mut subarray_count = 0;

    let _: Vec<i32> = modulo_array
        .iter()
        .map(|&item| {
            subarray_count += freq_array[item as usize];
            freq_array[item as usize] += 1;

            item
        })
        .collect();
    subarray_count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_problem() {
        let data = vec![4, 5, 0, -2, -3, 1];
        let k = 5;
        assert_eq!(7, get_subarray(data, k));
        let data = vec![5];
        let k = 9;
        assert_eq!(0, get_subarray(data, k));
    }
}
