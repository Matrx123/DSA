fn main() {
    println!("find the maximum for each window");
}

fn get_maximum(num: Vec<i32>, k: i32) -> Vec<i32> {
    let mut i = 0;
    let mut j = 0;
    let mut res = Vec::new();
    let n = num.len();
    let mut v = Vec::new();

    while j < n {
        if v.len() == 0 {
            v.push(num[j]);
        } else {
            while v.len() > 0 && v[v.len() - 1] < num[j] {
                v.remove(v.len() - 1);
            }
            v.push(num[j]);
        }

        if j - i + 1 == (k as usize) {
            res.push(v[0]);
            if v[0] == num[i] {
                v.remove(0);
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
    fn check_maximum() {
        let data = vec![1, 3, -1, -3, 5, 3, 6, 7];
        let k = 3;
        assert_eq!(vec![3, 3, 5, 5, 6, 7], get_maximum(data, k));
        let data = vec![1];
        let k = 1;
        assert_eq!(vec![1], get_maximum(data, k));
        let data = vec![1, 3, 1, 2, 0, 5];
        let k = 3;
        assert_eq!(vec![3, 3, 2, 5], get_maximum(data, k));
    }
}
