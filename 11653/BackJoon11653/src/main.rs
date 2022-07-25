use std::io::stdin;
use std::ops::Add;

fn main() {
    let mut input_str = String::new();
    stdin().read_line(&mut input_str).unwrap();
    let mut input_int = input_str.trim().parse::<i32>().unwrap();
    if input_int == 1 { return; }
    let mut prime: bool = true;
    let mut sqrt = f64::sqrt(input_int as f64);
    let sqrt: i32 = sqrt as i32;
    for i in 2..sqrt {
        if input_int % i == 0 { prime = false; break; }
    }
    if prime { println!("{}",input_int) }
    else {
        let mut num: i32 = 2;
        let mut result = String::new();
        while input_int != 1 {
            while input_int % num == 0 {
                input_int /= num;
                result.push_str(num.to_string().as_str());
                result.push("\n".parse().unwrap());
            }
            num+=1;
        }
        println!("{}",result)
    }
}
