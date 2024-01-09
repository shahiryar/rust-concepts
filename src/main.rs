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
    //let tup:(u8, i8, i8, i32) = (0, -2, -1, -432);
    //let zero = tup.0;
    //let minus_two = tup.1;
    //println!("{zero}");
    //println!("{minus_two}");

    //Array: Fixed size, single datatype for all elements, indexed and accessed via square bracket
    //Declare an array with 10 u32 values
    // let array:[u32;10] = [23, 4, 91, 5, 82, 82, 50, 34, 10, 76];
    //let el1 = array[1];
    //println!("{el1}");

    //Declare an array with size 12 filled with the values '3' in all places
    //let array=[3; 12];
    //let el12 = array[11];
    //println!("{el12}");

//----------------------------------------------------
//------FUNCTIONS-------------------------------------
//----------------------------------------------------
    //call_me();
    //let sum = sum_two_nums(12, 12);
    //println!("{sum}");
    //Expression versus Statements
    //Below is an example of assignment with an expression
    //let mut x = 10;
    //let y = x = 11;
    //println!("{x}"); 
    //Below is an example of a failed assignment with a statement
    //Expressions are evaluated while statements do not return a value
    //let x = (let y = 10);
    //Statement above will cause the compiler to produce error

    
    //A code block is also an expression
    //let y = {
    //    let x = 10;
    //    x + 1
    //};
    //Note in the statment above the block expression's last line
//----------------------------------------------------
//------CONDITIONS-------------------------------------
//----------------------------------------------------
    //if condition
    let x = 10;
    if x>11{
        println!("X is greater than 11");
    }else{
        println!("X is smaller than 11");
    }
    //Handling Multiple Conditions with else if
    let y = 12;
    if x > 11{
        println!("At least X is greater than 11");
    } else if y > 11{
        println!("X is not but Y is greater than 11");
    } else{
        println!("Neither X nor Y is greater than 11");
    }
    //if else are expressions which means that could be used to assign values to let
    let x = if x<11 { 12 } else { x };//return types of if else must be same!
    //let x = if x<11 { 12 } else { "Eleven" } // will return a compiler error
    println!("the value of x now is {x}");
//----------------------------------------------------
//------REPITIONS AND LOOPS---------------------------
//----------------------------------------------------
    //loop a number of times, without break statement loop runs indefinitly
    let mut count = 0;
    loop{
        count+=1;
        println!("Counting {count}");
        if count == 10{
            break
        }
    }
    
    //loop allows for returning a value as well
    //expression after break is returned
    //break acts as a return statement
    //if there is nothing after the break statments, unit () is returned
    let mut count = 0;//shadowing doesn't carry on the mutability
    let result = loop{
        count+=1;
        if count==10{
        break count;//semicolon here is optional!
        }
    };//note a semicolon after the loop whose value is bound to the variable!
    println!("Result returned from the loop is {result}");
   
    //Loop labels to disambiguate between multiple nested loops
    //break and continue in a nested loop apply to the outermost loop only
    //in order to specify which loop to break labels can be assigned to each loop
    let mut count = 0;
    'counting_up: loop{
        println!("Count is {count}");

        let mut remaining = 10;

        loop {
            println!("Remaining = {remaining}");
            if remaining == 9{
                break;
            }
            if count ==2{
                break 'counting_up;
            }
            remaining-=1;
        }
        count+=1
    }
    println!("End count = {count}");

    //While Loop
    let threshold = 10;
    let mut register = 0;
    while register<threshold{
        println!("Registered {register}");
        register+=1;
    }
    println!("");
    //Looping through List
    let mut index = 0;
    const ARRAY_SIZE:usize = 10;//can not be a variable! if is used to fix the array size
    let array:[i8;ARRAY_SIZE] = [10, 9, 2, 8, 12, 29, 67, 1, 0, 21];
    
    while index<ARRAY_SIZE{
        println!("Index {index} : {}",array[index]);
        index+=1;
    }
    

}

fn call_me(){
    println!("Outer Function was called!");
}

fn sum_two_nums(num1: i32, num2:i32)->i32{
    num1+num2
}
