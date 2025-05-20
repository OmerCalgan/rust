mod deneme;

struct Customer {
    name: String,
    surname: String,
    balance: f64,
}

struct Product {
    name: String,
    price: f64,
    stock_quantity: u32,
}

impl Customer {
    fn buy_product(&mut self, product: &mut Product, quantity: u32) -> bool {
        let total_price = product.price * quantity as f64;
        if product.stock_quantity >= quantity && self.balance >= total_price {
            self.balance -= total_price;
            product.stock_quantity -= quantity;
            true
        } else {
            false
        }
    }
}

fn main() {
    let mut customer1 = Customer {
        name: String::from("John"),
        surname: String::from("Doe"),
        balance: 100.0,
    };

    let mut customer2 = Customer {
        name: String::from("Jane"),
        surname: String::from("Doe"),
        balance: 50.0,
    };

    let mut product = Product {
        name: String::from("Laptop"),
        price: 30.0,
        stock_quantity: 10,
    };

    println!("Customer 1 is buying a product...");
    if customer1.buy_product(&mut product, 3) {
        println!("Customer 1 successfully purchased the product.");
    } else {
        println!("Customer 1 couldn't purchase the product.");
    }

    println!("Customer 2 is buying a product...");
    if customer2.buy_product(&mut product, 2) {
        println!("Customer 2 successfully purchased the product.");
    } else {
        println!("Customer 2 couldn't purchase the product.");
    }
}
