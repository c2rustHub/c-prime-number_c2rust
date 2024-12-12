use std::f64;

fn main() {
    let mut max = 0;

    println!("max prime from 1 to 100");

    for num in (1..=100000).step_by(2) {
        let tmp = (num as f64).sqrt() as i32;

        let mut is_prime = true;
        for i in 2..=tmp {
            if num % i == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            max = num;
        }
    }

    println!("max prime is {}", max);
}
