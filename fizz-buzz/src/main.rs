fn main() {
    println!("Hello, from fizz buzz!");
    let mut fizzbuzz = 0;
    for i in 1..302 {
        match (i % 3, i % 5) {
            (0, 0) => { 
                fizzbuzz += 1;
                println!("{}\tfizz buzz", i);
            }
            (0, _) => println!("{}\tfizz", i),
            (_, 0) => println!("{}\tbuzz", i),
            _ => println!("{}", i)       
        }
    }
    println!("\n\nfizz buzz occured {} times", fizzbuzz);
}
