use std::io;


fn main() {
    
    println!("Type the Fibonnaci Sequence Number:");
    let mut number = String::new();

    io::stdin()
    .read_line(&mut number)
    .expect("Failed to read number");

    let number: i32 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => -1,
    };

    if number > 0{

        let mut first = 0;
        let mut second = 1;
        let mut counter = 1;

        println!("{first}");
        println!("{second}");

        let result = loop {

            if number == 1{
                break first;
            }
            if number == 2{
                break second;
            }

            let third = first + second;
            first = second;
            second = third;
            println!("{second}");
            
            if counter == (number-2) {
                break third;
            }

            counter += 1;
        };
        println!("The number {result} is the {number} th of the Fibonnaci Sequence")
        
    }else{
        println!("Invalid number typed");
    }


}
