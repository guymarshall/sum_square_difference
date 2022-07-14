pub fn number_to_vector(number: u128) -> Vec<u128> {
    let mut numbers: Vec<u128> = Vec::new();

    for i in 1..number + 1 {
        numbers.push(i);
    }

    return numbers;
}

pub fn square_vector_values(values: &Vec<u128>) -> Vec<u128> {
    let mut squared_values: Vec<u128> = Vec::new();

    for value in values.iter() {
        squared_values.push(value * value);
    }

    return squared_values;
}