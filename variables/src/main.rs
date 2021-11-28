fn main() {
    let x = 5;
    println!("The value of x is {}", x);
    let x = x + 6;
    {
        let x = x + 11;
        println!("The value of x in the inner scope is {}", x);
    }
    println!("The value of x is {}", x);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;

    println!("The value of y is: {}", y);
    println!("The value of z is: {}", tup.2);
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("The value of first index of array is: {}", a[1]);

}
