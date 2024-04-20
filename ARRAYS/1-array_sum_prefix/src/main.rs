fn main() {
    println!("Get Highest altitude using prefix sum");

    let result = prefix_sum_array([-4, -3, -2, -1, 4, 3, 2]);
    println!("results >>> {:?}", result);
}

fn prefix_sum_array(arr: [i32; 7]) -> Vec<i32> {
    let mut new_arr: Vec<i32> = Vec::new();

    let mut sum = 0;
    let mut highest = 0;
    for item in arr {
        sum += item;
        if sum > highest {
            highest = sum;
        }
        new_arr.push(sum);
    }
    println!("highest altitude : {:?}", highest);
    new_arr
}
