//const YEAR_OF_INDEPENDENCE:u32 = 1947;
fn main() {
//------- VARIABLES & CONSTANTS --------------
    // Variables by default are immutable
    // using mut keyword a variable can be made mutable
    //let y = 12;
    //y is an immutabel variable; no new value can be bound to it
    //let mut x = 10;
    //println!("The value of x is {x}");
    //x = 5;
    //println!("The value of x is {x}");
    //println!("The Year Pakistan was born on is {YEAR_OF_INDEPENDENCE}");
//----------------------------------------------
//------- SHADOWING  ---------------------------
//----------------------------------------------
    //Creating a immutable variable
   let x = 10;
   println!("value of x in main scope before it is overshadowed : {x}");
   let x = x+1; //over-shadowing variable
   {
       let x = 23;
       println!("Shadowed x in local scope {x}");
    }
   println!("out of scope shadowed {x}");
}
