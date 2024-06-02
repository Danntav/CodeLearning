fn sum (number1: i32, number2: i32) -> i32{
    number1 + number2
}

fn display_result (result: i32){
    println!("The result is {:?}", result);
}


fn main(){
    let plus = sum(2,3);
    display_result(plus);
}