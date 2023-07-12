pub struct Pizza {
    pub dough: String,
    pub cheese: String,
    pub topping: String,
}

impl Pizza {
    pub fn lunch(topping: &str) -> Pizza {
        return Pizza {
            dough: String::from("Regular Dough"),
            cheese: String::from("Mottizorilla"),
            topping: String::from(topping),
        };
    }
}

fn seat_at_table() {
    println!("Customer seated at table");
}
pub fn take_order() {
    seat_at_table();
    serve_customer(Pizza::lunch("Veggies"));
}

fn serve_customer(cust_pizza: Pizza) {
    println!(
        "The Customer is served a regular pizza with {}",
        cust_pizza.topping
    );
}

pub fn order_food() {
    crate::restaurant::take_order()
}
