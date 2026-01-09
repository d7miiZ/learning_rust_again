use std::io;

fn main() {
    println!("Enter the number of terms:");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: u32 = n.trim().parse()
        .expect("Please type a number!");

    println!("The Fibonacci sequence up to {n} terms is:");
    let mut a = 0;
    let mut b = 1;
    for i in 0..n {
        if i == 0 {
            print!("0 ");
        } else if i == 1 {
            print!("1 ");
        } else {
            let next = a + b;
            a = b;
            b = next;
            print!("{next} ");
        }
    }
    println!();
}
