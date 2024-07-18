fn is_even(n: &i32) -> bool
{
    if n % 2 == 0
    {
        true
    }
    else
    {
        false
    }
}


fn main() {
    let arr : [i32;10] = [2, 10, 13, 22, 29, 33, 43, 54, 61, 69];

    for num in arr.iter()
    {
        if num % 3 == 0
        {
            println!("Fizz");
        }
        else if num % 5 == 0
        {
            println!("Buzz")
        }
        else if num % 5 == 0 && num % 3 == 0
        {
            println!("FizzBuzz")
        }
        else
        {
            if is_even(num) == true
            {
                println!("even");
            }
            else
            {
                println!("odd");
            }
        }
        
    }

    let mut counter = 0;
    let mut sum = 0;
    while counter != 10 {
        sum += arr[counter];
        counter += 1;

    }
    println!("{}", sum);

    let mut num = 0;
    let mut largest_num = 0;
    loop{
        largest_num = arr[num];
        num += 1;
        if arr[num] > arr[num - 1]
        {
            largest_num = arr[num];
        }
        else
        {
            largest_num = arr[num-1];
        }
    
    if num == 10{
        break;
    }
}
    println!("{}", largest_num);
}
