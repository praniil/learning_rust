trait Vehicle {
    fn mileage(&self) -> String;
}

struct Sadan {
    brand: String,
    price_in_usd: u32,
    mileage: u32,
}

impl Vehicle for Sadan {
    fn mileage(&self) -> String {
        format!("{} has mileage of {} kms per ltr", self.brand, self.mileage)
    }
}

impl Sadan {
    fn get_price(&self) -> u32 {
        self.price_in_usd
    }
}

struct Trucks {
    brand: String,
    weight_in_kgs: u32,
    mileage: u32,
}

impl Vehicle for Trucks {
    fn mileage(&self) -> String {
        format!("this truck from {} ({} kgs) gives {} mileage", self.brand, self.weight_in_kgs, self.mileage)
    }
}

//static
fn print_mileage<T: Vehicle>(vehicles: &T) {
    println!("{}", vehicles.mileage())
}

//dynamic
// fn print_mileage(vehicles: &dyn Vehicle) {
//     println!("{}", vehicles.mileage())
// }


pub fn cars() {
    let merc_benz_s_class = Sadan{
        brand: String::from("Mercedes"),
        price_in_usd: 230000,
        mileage: 10, 
    };

    let actros_l = Trucks{
        brand: String::from("Mercedes"),
        weight_in_kgs: 4353,
        mileage: 6,
    };

    merc_benz_s_class.get_price();

    print_mileage(&actros_l);
    print_mileage(&merc_benz_s_class);
}