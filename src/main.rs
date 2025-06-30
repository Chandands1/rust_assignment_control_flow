

fn main() {
    println!("Hello, world!");

    let result = color_to_number("orange");

    println!("{result}");

    let factorial_result = factorial(10);

    println!("The factorial is {factorial_result}");

    println!("the factorial is {} ", factorial_recurstion(5))

}

fn color_to_number(color: &str) -> i32 {
//     if color=="red"{
//         return 1;
//     }else if color == "green"{
//         return 2;
// }
// else if color == "blue"{
//     return 3;
// }
// else{
//     return 0;
// }

match color {

    "red" => 1,
    "green" => 2,
    "blue" => 3,
    _ => 0
    
}
}

fn factorial(number: i32) -> i32{
    let mut fact =1;
    for n in  1..=number{
        fact = fact * n;
    }
    fact
}

fn factorial_recurstion(number: i32) -> i32{
    match number {

        0 | 1 => 1,
         _ => number * factorial(number -1)
        
    }

}
