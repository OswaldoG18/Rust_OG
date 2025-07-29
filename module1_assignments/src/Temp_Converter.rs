const FREEZING_POINT: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64{
    let q = 5.0 / 9.0; 
    (f - FREEZING_POINT) * q
}

fn celsius_to_fahrenheit(c: f64) -> f64{
    let w = 9.0 / 5.0; 
    (c * w) + FREEZING_POINT
}

fn main() {
    let mut x = 32.0;
    let end_loop = x + 6.0;
        println!("Conversion from {}°F Fahrenheit to Celsius = {}°C",x,fahrenheit_to_celsius(x));
    loop {
        x += 1.0;
        if x == end_loop {
            break;
        }
       println!("Conversion from {}°F Fahrenheit to Celsius = {}°C",x,fahrenheit_to_celsius(x));
    }
}


