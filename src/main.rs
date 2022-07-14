mod math;
mod sum;
mod user_input;

fn main() {
    let number: u128 = user_input::get_user_input("Number:");
    let numbers: Vec<u128> = math::number_to_vector(number);
    let squared_values: Vec<u128> = math::square_vector_values(&numbers);
    let sum_of_squared_values: u128 = sum::sum(&squared_values);
    let sum_of_values: u128 = sum::sum(&numbers);
    let squared_sum: u128 = sum_of_values.pow(2);
    let difference: u128 = squared_sum - sum_of_squared_values;

    println!("Difference between {} and {} is: {}", squared_sum, sum_of_squared_values, difference);
}