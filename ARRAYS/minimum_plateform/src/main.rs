fn main() {
    println!("Minimum Number of Platforms Required for a Railway/Bus Station");
}

fn get_minimum_plateform(arrival: Vec<i32>, depart: Vec<i32>) -> i32 {
    let arrival_max = arrival.iter().max().unwrap_or(&0);
    let depart_max = depart.iter().max().unwrap_or(&0);
    //as the arrival and departure are the time values, maximum of it can be 23:59 or 2359.
    //but i choose to find the maximum of both the vectors.
    let max_size = if arrival_max > depart_max {
        (arrival_max + 1) as usize
    } else {
        (depart_max + 1) as usize
    };
    let mut v = vec![0; max_size];

    let _: Vec<i32> = arrival
        .iter()
        .enumerate()
        .map(|(index, &item)| {
            v[item as usize] += 1;
            v[depart[index as usize] as usize] -= 1;
            item
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
    *ps.iter().max().unwrap_or(&0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_minimum_plateform() {
        //Input: arr[] = {9:00, 9:40, 9:50, 11:00, 15:00, 18:00}, dep[] = {9:10, 12:00, 11:20, 11:30, 19:00, 20:00}
        let input = vec![900, 940, 950, 1100, 1500, 1800];
        let output = vec![910, 1200, 1120, 1130, 1900, 2000];
        assert_eq!(3, get_minimum_plateform(input, output));
        let input = vec![900, 940];
        let output = vec![910, 1200];
        assert_eq!(1, get_minimum_plateform(input, output));
    }
}
