fn main() {
    // Variables by default are immutable
    // using mut keyword a variable can be made mutable
    let y = 12;
    //y is an immutabel variable; no new value can be bound to it
    let mut x = 10;
    println!("The value of x is {x}");
    x = 5;
    println!("The value of x is {x}");
}
