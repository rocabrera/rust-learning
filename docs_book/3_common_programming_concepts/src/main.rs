fn assign_error() {
    let x = 5;
    println!("The value of x is: {x}");
    // x = 6; // cannot assign twice to immutable variable
}

fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn mut_vs_shadowing(){
    let spaces = "   ";
    let spaces = spaces.len();

    // Cannot change type of mut variable
    // let mut spaces = "   ";
    // spaces = spaces.len();

}

fn main() {


    println!("Running Assign error:");
    assign_error();

    println!("Running shadowing:");
    shadowing();

    println!("Running mut vs shadowing:");
    mut_vs_shadowing();
}

