mod tuple;
mod structs;
mod arrays;
mod practice_2;
fn main (){
   
    // String Interpolation
    // 1. printlin! 2. format! 3. panic!

    // tuple::tuple();
    // structs::structs();
    // arrays::arrays();

    // Exercises
    // practice_2::main();
    let is_test:bool = testme(true);
    println!("value {}",is_test);
   

}

fn testme(is_test:bool)->bool{
    if is_test{
    return is_test
    }
    return is_test
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

