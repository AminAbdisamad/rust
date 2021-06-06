fn main (){
    let greeting = "Hello there";
    let name = "Hassan";
    // String Interpolation
    // 1. printlin! 2. format! 3. panic!
    println!("{}, {}", greeting, name);
    format!("{}, {}", greeting,name);

    let _server_message = "Server is down";
    // panic!("Its happening {} ", server_message);
    println!("This will NOT run");

    let result = multiply(2.1, 7.9);
    println!("Result is {}", result)



}

fn multiply(x:f64, y:f64)->f64{
    let name = "hassan";
    if name.starts_with('h'){
        println!("{} starts with (h) ",name)
    }else{
        println!("{} Not start with  (h) ",name)
    }
return x*y;
}