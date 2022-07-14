pub fn sum(numbers: &mut Vec<u128>) -> Vec<u128> {
    let mut sum: u128 = 0;

    for_each!(number in numbers {
        sum += number;
    });

    return sum;
}