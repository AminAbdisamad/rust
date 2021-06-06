struct Point {
    x:f64,
    y:f64,
    z:f64,
}

struct UserInfo {
    name:String,
    email:&'static str,
    phone:i64,
}

pub fn structs(){
    let user_info = UserInfo{name:"Hassan".to_string(),email:"Hassan@gmail.com",phone:233434546};
    let UserInfo {name,email,..} = user_info;

    let  phone = user_info.phone;
    
    println!("Name:{} Email:{} phone:{}", name,email,phone);
    
let point = Point{x:1.0,y:2.4,z:3.6};
//    Getting values out of struct
let x = point.x;
let y = point.y;
let z = point.z;
println!("x={}, y={} z={}",x,y,z);
// Or destructure

let Point {x,y,z} = point;
// if you want to ignore an item use _underscore
let Point {x,y,z:_} = point;
// if you want to ignore more than one item
let Point {x,..} = point; // this means ignore all the other fields 

// Mutable structs
let mut points = Point{x:1.0,y:2.4,z:3.6};
points.x=5.0;
 

}

fn _new_point (x:f64,y:f64,z:f64)->Point{
    Point{x:x,y:y,z:z};
    // you can also do this 
    Point{x,y,z}
}
