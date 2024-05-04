fn main() {
    println!("Maximum Population year");
}

fn get_max_population(logs: Vec<Vec<i32>>) -> i32 {
    let mut v = vec![0; 2011];
    let _data: Vec<&Vec<i32>> = logs
        .iter()
        .map(|inner_vec| {
            v[inner_vec[0] as usize] += 1;
            v[inner_vec[1] as usize] -= 1;
            inner_vec
        })
        .collect();
    let mut ps = Vec::new();
    let mut sum = 0;
    let _: Vec<i32> = v
        .iter()
        .map(|&item| {
            sum += item;
            ps.push(sum);
            item
        })
        .collect();
    let max = *ps.iter().max().unwrap_or(&0);
    match ps.iter().position(|&x| x == max) {
        Some(index) => index.try_into().unwrap(),
        None => 0,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_max_population() {
        let data1 = vec![vec![1993, 1999], vec![2000, 2010]];
        let data2 = vec![vec![1950, 1961], vec![1960, 1971], vec![1970, 1981]];
        assert_eq!(1993, get_max_population(data1));
        assert_eq!(1960, get_max_population(data2));
    }
}
