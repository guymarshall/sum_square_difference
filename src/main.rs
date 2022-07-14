mod math;
mod sum;
mod user_input;

fn main() {
    println!("Hello, world!");
}

/*

CONVERT TO RUST

int main(int argc, char const *argv[])
{
    std::cout <<"Number: ";
    long long number;
    std::cin >> number;

    std::vector<long long> numbers = numberToVector(number);

    std::vector<long long> squaredValues = squareVectorValues(numbers);
    long long sumOfSquaredValues = sum(squaredValues);
    long long sumOfValues = sum(numbers);

    long long squaredSum = pow(sumOfValues, 2);
    long long difference = squaredSum - sumOfSquaredValues;
    
    std::cout << "Difference between " << squaredSum << " and " << sumOfSquaredValues << " is: " << difference << std::endl;
    return 0;
} */