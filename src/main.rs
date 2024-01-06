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
   //let x = 10;
   //println!("value of x in main scope before it is overshadowed : {x}");
   //let x = x+1; //over-shadowing variable
   //{
      // let x = 23;
       //println!("Shadowed x in local scope {x}");
    //}
   //println!("out of scope shadowed {x}");
//----------------------------------------------
//---------DATA TYPES--------------------------
//----------------------------------------------
    //Scaler data types: data types that contain a single value
    //Rust has 4 primary scaler data types
    // Intergers, Floating-Numbers, Booleans, and Character

    //Interger Types
    //Integer could be unsigned (staring with u) or signed (starting with i)
    //u8, u16, u32, u64, u128, usize
    //let x:u32 = 123;
    //i8, i16, i32, i64, i128, isize
    //let y:i32 = -23;
    //println!("{x}, {y}");
    
    //Floating point number
    //let z: f32 = 2.3;
    //println!("{z}");

    //NUMERIC OPERATIONS
    //let sum = 3+2;
    //let difference = 2-1;
    //let product = 8*3;
    //let quotient  = 56.7/32.2;
    //let truncated = -5/3;
    //let remainder = 3%2;

    //println!("{sum}");
    //println!("{difference}");
    //println!("{product}");
    //println!("{quotient}");
    //println!("{truncated}");
    //println!("{remainder}");

//------COMPOUND DATATYPES-------------------
    //Tuples: Fixed size, various datatypes, indexed and accessed via dot index
    let tup:(u8, i8, i8, i32) = (0, -2, -1, -432);
    let zero = tup.0;
    let minus_two = tup.1;
    println!("{zero}");
    println!("{minus_two}");

    //Array: Fixed size, single datatype for all elements, indexed and accessed via square bracket
    //Declare an array with 10 u32 values
    let array:[u32;10] = [23, 4, 91, 5, 82, 82, 50, 34, 10, 76];
    let el1 = array[1];
    println!("{el1}");

    //Declare an array with size 12 filled with the values '3' in all places
    let array=[3; 12];
    let el12 = array[11];
    println!("{el12}");
    
    
}
