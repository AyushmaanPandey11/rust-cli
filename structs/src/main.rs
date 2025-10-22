use std::env;

struct Rectangle {
    length: i16,
    width: i16,
}

impl Rectangle {
    fn area( input: &Rectangle ) -> i16 {
        return input.length * input.width;
    }
}

fn main() {
    // 1. Get arguments as Vec<String> and check for correct count.
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <length> <width>", args[0]);
        // Exit the program if the wrong number of arguments are provided
        return; 
    }

    let length: i16 = args[1].parse().expect("Length argument is not a valid i16");
    let width: i16 = args[2].parse().expect("Width argument is not a valid i16");
    let input: Rectangle = Rectangle{ length: length, width: width };
    let ans = Rectangle::area(&input);
    println!("area is {}",ans);


}
