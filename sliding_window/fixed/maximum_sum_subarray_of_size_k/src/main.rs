fn main() {
    println!(
        "Given an array of integers Arr of size N and a number K. Return the maximum sum of a subarray of size K."
    );
}

fn get_max_subarray_sum(arr: Vec<i32>, k: i32, n: i32) -> i32 {
    let mut i = 0;
    let mut j = 0;
    let mut sum = 0;
    let mut max_sum = 0;
    while j < n {
        sum += arr[j as usize];
        if j - i + 1 == k {
            if sum > max_sum {
                max_sum = sum;
            }
            sum -= arr[i as usize];
            i += 1;
            j += 1;
        } else {
            j += 1;
        }
    }

    max_sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_max_subarray_sum() {
        let arr = vec![100, 200, 300, 400];
        let n = 4;
        let k = 2;
        assert_eq!(700, get_max_subarray_sum(arr, k, n));
        let arr = vec![100, 200, 300, 400];
        let n = 4;
        let k = 4;
        assert_eq!(1000, get_max_subarray_sum(arr, k, n));
    }
}
