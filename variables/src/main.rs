fn main() {
    let x = 5;
    println!("The value of x is {}", x);
    let x = x + 6;
    {
        let x = x + 11;
        println!("The value of x in the inner scope is {}", x);
    }
    println!("The value of x is {}", x);

}