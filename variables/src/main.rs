fn main() {
    let mut x = 5;
    println!("The value of x is {}",x);
    x = x+1;
    println!("The value of x is {}",x);
    let a = [3;5];
    // use Debug mode to print array of int
    // type must support Debug. otherwise, this will not compile
    println!("{:?}",a);
}
