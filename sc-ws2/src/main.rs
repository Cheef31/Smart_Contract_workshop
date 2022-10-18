fn main() {
    let mut inputint: u32 = 1;
    loop {
        if inputint == 100 {
            println!("Arrived at {} --> Finished!", inputint);
            break
        }
        if inputint % 15 == 0{
            println!("fizz buzz");
        } else {
            if inputint % 3 == 0 {
                println!("fizz");
            }
            else if inputint % 5 == 0 {
                println!("buzz");
            } else {
                println!("'{inputint}'");
            }
        }
        inputint += 1;
    }
}