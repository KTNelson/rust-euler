//If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
//
//Find the sum of all the multiples of 3 or 5 below 1000.


fn main() {
    
    let max_numbers = 1000i;

    let mut counter = 1;
    let mut result = 0;

    while(counter < max_numbers)
    {
    	if(counter % 3 == 0 || counter % 5 == 0)
    	{
    		result += counter;
    	}

    	counter += 1;
    }

    println!("Result is: {}", result)
}
