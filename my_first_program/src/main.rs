fn fahrenheit_to_celsius(f: f64) -> f64
{
    (f - FREEZING_POINT) * (5.0/9.0)
}

fn celsius_to_fahrenheit(f: f64) -> f64
{
    (f * (9.0/5.0)) + FREEZING_POINT
}




const FREEZING_POINT: f64 = 32.0;

fn main() {
    
    let mut tempf = 86.0;
    
    println!("{}", fahrenheit_to_celsius(tempf));
    
    let mut tempf2 = 32.0;
    let mut num = 0;
    loop
    {
        println!("{}", fahrenheit_to_celsius(tempf2));
        num += 1;
        tempf2 += 1.0;

        if num == 5
        {
            break;
        }

    }



}