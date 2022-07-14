pub fn number_to_vector(number: u128) -> Vec<u128> {
    let mut numbers = Vec::new();

    for i in 1..number + 1 {
        numbers.push(i);
    }

    return numbers;
}

pub fn square_vector_values(values: &mut Vec<u128>) -> Vec<u128> {
    let mut squared_values = Vec::new();

    for i in 1..values.len() {
        squared_values.push(values[i] ** 2);
    }

    return squared_values;
}