use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let max_value_str = args.get(1).expect("\n\tthere is no max value provided");
    let max_value:i64 = max_value_str.parse().expect("\n\tnot a integer");
    let sum = sum_of_natural_number(max_value);
    println!("Sum is {}", sum)
}


fn sum_of_natural_number(max_value : i64) -> i64 {
    let mut sum = 0;
    for val in 1..max_value{
        if val % 15 == 0 {
            sum = sum + val;
        } else if  val % 5 == 0 {
            sum = sum + val;
        } else if  val % 3 == 0 {
            sum = sum + val;
        }
    }
    sum
}

#[test]
fn test_sum_of_natural_number(){
    let sum = sum_of_natural_number(10);
    assert_eq!(sum, 23);
}