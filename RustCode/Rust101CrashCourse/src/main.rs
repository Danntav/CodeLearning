
// This is the entry point of the
// application.
fn add (a: i32, b:i32) -> i32 {
    a + b
}

    //Display a message to the user.
fn main(){
let _x = add(1,1);
let _y = add(3,0);
let _z = add(_x,1);


let my_favorite_color = "red";

let life = 42;
println!("hello");
println!("{:?}", life);
println!("{:?} {:?}", life, life);
println!("the meaning is {:?}", life);

println!("{life:?}");
println!("{life}");

let a = 99;
if a > 200{
    println!("Huge number");
} else if a > 99{
    println!("Big number");
} else{
    println!("Small number");
}


let mut a = 0;
loop {
    if a == 5 {
        break;
    }
    println!("{:?}", a);
    a += 1;
}


let mut a = 0;
while a != 5 {
    println!("{:?}", a);
    a += 1;
}



}