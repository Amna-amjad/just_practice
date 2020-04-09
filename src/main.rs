// #[derive(Debug)]
// enum Direction{          // 1. Make an enum Direction.
//                           //    with variants forward,backward,left,right.
//     forward,
//     backward,
//     left,
//     right,
// }
// fn main(){
//     check_directions(Direction::backward);     // 2. Make a user defined function with name "check_direction".
// }
// fn check_directions(x: Direction){             //    That takes a variable "x" of Data Type "Direction" as parameter.
//     match x {                                                                // 3. Inside the body of the function "check_direction".
//         Direction::forward => println!("Car is moving forward" ),              // Handle all cases with "match" for example.
//         Direction::backward => println!("Car is moving backward" ),     
//         Direction::left => println!("Car is moving left" ),
//         Direction::right => println!("Car is moving right" ),
//     }

// }


// fn main(){
//     let mut s = String::from("Pakistan"); // this is how we can push a string into another one through push_str.
//     change(&mut s);
//     println!("{}",s);
// }
// fn change (some_string: &mut String){
//     some_string.push_str( " Zindabad");
// }



// fn main() {
//     let mut string = String::from ("Stay Home");   //same as above example
//     include(&mut string);
//     println!("{}",string );
// }
// fn include(some_string:&mut String) {
//     some_string.push_str(" Stay safe");
// }
    // fn main(){                            // code for userinput for  finding average of num
    //      let num1 = input_f32("num1");
    //      let num2 = input_f32("num2");
    //      let num3 = input_f32("num3");
    //      let average = (num1+num2+num3)/3.0;
    //      println!("Average of number is: {}",average );
    // }

    // pub fn input_f32(string_literal: &str)-> (f32) {
    //     println!("Enter {}",string_literal );
    //     let mut input = String::new();
    //     std::io::stdin().read_line(&mut input).expect("Input Failed");
    //     let integer:f32 = input.trim().parse().unwrap();
    //     integer
    // }

    fn main(){
        let input_str = input("Enter you input please:"); //input fun for finding length of string
        let (s2, length) = calculate_len(input_str);
        println!("length is {} of the string: {}",length,s2 );

    }
    fn calculate_len(input_str: String) -> (String, usize){
        let length = input_str.len()-1;
        (input_str, length)
    }
    fn input(string_literal: &str)-> (String){
        println!("Enter {}",string_literal );
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Input Failed");
        input
    }







































