pub fn non_constructible_change(mut coins: Vec<i32>) -> i32 {
    coins.sort();
    let mut current_change_created = 0;
    for coin in coins {
        if coin > current_change_created + 1 {
            return current_change_created + 1;
        }
        current_change_created += coin;
    }
    return current_change_created + 1;
}

fn main() {
    let test_arr = vec![5, 7, 1, 1, 2, 3, 2, 2];
    println!("{}", non_constructible_change(test_arr));
}
