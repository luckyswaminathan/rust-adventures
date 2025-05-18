use std::io;

fn main() {
    let mut n = String::new();
    //read
    println!("enter the n value for the fibonacci number you want");
    io::stdin().read_line(&mut n)
    .expect("Failed to read line");
    //shadow n
    let n: u32 = n.trim().parse()
    .expect("Please type a number!");
    let mut prev = 1;
    let mut prev_prev = 1;
    for _ in 2..n {
        // value that only exists in scope of each iteration
        let temp = prev_prev;
        prev_prev = prev;
        prev = prev + temp;
    }

    println!("nth fibonacci number {}", prev);


}
