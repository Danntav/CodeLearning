fn first_name(first: &str){
    println!("Your first name: {:?}", first);
}

fn last_name(last: &str){
    println!("Your last name: {:?}", last);
}

fn main(){

    let a: &str = "Daniel";
    let b: &str = "Tavares";
    first_name(a);
    last_name(b);
}