
fn is_even(n: i32) -> bool {
    n % 2 == 0 
}


fn main() {
   
    let nums = [18, 22, 34, 5, 3, 8, 90, 43, 75, 68];

    let mut output: Vec<String> = Vec::new();
    let mut even_odd: Vec<String> = Vec::new();
    

    for &i in nums.iter(){
        if is_even(i){
            even_odd.push("True".to_string());
        }else{
            even_odd.push("False".to_string());
        }
    }

    for &i in nums.iter() {
        if i % 3 == 0 && i % 5 == 0 {
            output.push("FizzBuzz".to_string());
        } else if i % 3 == 0 {
            output.push("Fizz".to_string());
        } else if i % 5 == 0 {
            output.push("Buzz".to_string());
        } else {
            output.push(i.to_string());
        }
       }

    let mut total_sum = 0;
    let mut idx = 0;
    while idx < nums.len(){
        total_sum += nums[idx];
        idx += 1;
    }

    let mut large_num = nums[0];
    for &i in nums.iter(){
        if i > large_num{
            large_num = i;
        }
    

    }


    println!("{:?}",nums);
    println!("");
    println!("{:?}",even_odd);
    println!("{:?}",output);
    println!("Sum of all numbers in array: {}",total_sum);
    println!("Largest Number in array: {}",large_num);

    

fn check_guess(guess: i32, secret: i32) -> i32 { 
    
    if guess == secret{
       return 0;
    } else if guess > secret{
        return 1;
    } else {
        return -1;
    }
    
    
}


fn main() {

    let mut secret = 10; 
    let mut guess = 0; 
    let mut attempts = 0;

    

    loop {

        attempts += 1;
        let answer = check_guess(guess, secret);

        if answer == 0{
            println!("Correct: {}",secret);
            break;
        }
        else if answer == 1 {
            println!("Guess to High: {}",guess);
            guess -= 1
        }
        else if answer == -1 {
            println!("Guess to Low: {}", guess);
            guess += 1
        }
    }

    println!("Number of Attempts = {}",attempts);
}

    
}