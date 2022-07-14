mod math;
mod sum;
mod user_input;

fn main() {
    let number: u128 = user_input::get_user_input("Number:");
    let numbers: Vec<u128> = math::number_to_vector(number);
}

/*

CONVERT TO RUST

std::vector<long long> squaredValues = squareVectorValues(numbers);
long long sumOfSquaredValues = sum(squaredValues);
long long sumOfValues = sum(numbers);

long long squaredSum = pow(sumOfValues, 2);
long long difference = squaredSum - sumOfSquaredValues;

std::cout << "Difference between " << squaredSum << " and " << sumOfSquaredValues << " is: " << difference << std::endl;
return 0;*/