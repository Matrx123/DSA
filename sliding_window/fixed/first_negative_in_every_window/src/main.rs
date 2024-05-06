fn main() {
    println!("First negative integer in every window of size k");
}

fn get_first_negative(arr: Vec<i32>, k: usize) -> Vec<i32> {
    let mut i = 0;
    let mut j = 0;
    let n = arr.len();
    let mut v = Vec::new();
    let mut res = Vec::new();
    while j < n {
        if arr[j as usize] < 0 {
            v.push(arr[j as usize]);
        }
        if j - i + 1 == k {
            if v.len() == 0 {
                res.push(0);
            } else {
                res.push(v[0]);
                if arr[i] == v[0] {
                    v.remove(0);
                }
            }
            i += 1;
            j += 1;
        } else {
            j += 1;
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_first_negative() {
        let arr = vec![-8, 2, 3, -6, 10];
        let k = 2;
        assert_eq!(vec![-8, 0, -6, -6], get_first_negative(arr, k));
    }
}
