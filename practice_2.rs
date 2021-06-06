struct City {
    description: String,
    residents: u64,
    is_coastal:bool,
}

fn new_city(residents: u64, is_coastal: bool) -> City {
    if is_coastal {
        City {
            description: format!("a *coastal* city of approximately {} residents", residents),
            residents,
            is_coastal,
        }
    } else {
        City{
            description: format!("a *non-coastal* city of approximately {} residents",residents),
            residents,
            is_coastal,
        }
        // panic!(
        //     "👉 TODO return a `City` described as a *non-coastal* city of approximately {} residents"
            
        // );
    }
}


pub fn main() {
    let rustville = new_city(100_000, false);
//    City {description:" Amazing city".to_string(), residents:100_000,is_coastal:true};
    // let description = rustville.description;
    let City {description,..} = rustville;

    println!("This city can be described as: {}",description);

    if rustville.is_coastal {
        println!("It is a coastal city.");
    } else {
        println!("It is not a coastal city.");
    }
}