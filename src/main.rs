use anyhow::Result;
use shopping_parser::{
    load_products_from_json, parse_products_file, save_products_to_json, Product,
};
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_info();
        return Ok(());
    }

    match args[1].as_str() {
        "--parse" => {
            if args.len() < 4 {
                eprintln!("Error: Missing input or output file path.");
                print_info();
                return Ok(());
            }
            let input_path = &args[2];
            let output_path = &args[3];
            let products = parse_products_file(input_path)?;
            save_products_to_json(&products, output_path)?;
        }
        "--get" => {
            if args.len() < 3 {
                eprintln!("Error: Missing product name.");
                print_info();
                return Ok(());
            }
            let product_name = &args[2];
            let products = load_products_from_json("./src/output.json")?;
            if let Some(product) = products.iter().find(|p| p.product_name == *product_name) {
                display_product(product);
            } else {
                println!("Product '{}' not found.", product_name);
            }
        }
        "--list" => {
            if args.len() < 3 {
                eprintln!("Error: Missing shopping list.");
                print_info();
                return Ok(());
            }
            let shopping_list = &args[2..].join(", ");
            let products = load_products_from_json("./src/output.json")?;
            calculate_total_cost(shopping_list, &products);
        }
        "--credits" => {
            display_credits();
        }
        "--help" => print_info(),
        _ => {
            eprintln!("Unknown command.");
            print_info();
        }
    }

    Ok(())
}

fn print_info() {
    println!("Usage:");
    println!("  --parse <input_file> <output_file> - Parse and save products to JSON.");
    println!("  --get <product_name>              - Display product information.");
    println!("  --list <shopping_list>            - Calculate total cost of shopping list.");
    println!("  --credits                         - Display credits information.");
    println!("  --help                            - Show help information.");
}

fn display_product(product: &Product) {
    println!("Product name: {}", product.product_name);
    println!("Category: {}", product.category);
    println!("Price: {} {}", product.price_per_unit, product.unit);
    println!("Calories: {} cal", product.calories);
    println!("Proteins: {} g", product.proteins);
    println!("Carbohydrates: {} g", product.carbohydrates);
    println!("Fats: {} g", product.fats);
}

fn display_credits() {
    println!("Credits:");
    println!("This shopping parser was developed by Oleksandr Durdynets.");
    println!("Version: 1.0.0");
    println!("Thank you for using my parser!");
    println!("Glory to Ukraine!");
}

fn calculate_total_cost(shopping_list: &str, products: &[Product]) {
    let mut total_cost = 0.0;

    let items: Vec<&str> = shopping_list.split(',').collect();

    for item in items {
        let parts: Vec<&str> = item.split_whitespace().collect();
        if parts.len() < 3 {
            println!("Invalid item format. Expected: <name> <quantity> <unit>");
            continue;
        }

        let product_name = parts[0];
        let quantity: f64 = parts[1].parse().unwrap_or(0.0);
        let unit = parts[2];

        if let Some(product) = products.iter().find(|p| p.product_name == product_name) {
            let product_unit_parts: Vec<&str> = product.unit.split('/').collect();
            if product_unit_parts.len() == 2 && product_unit_parts[1] == unit {
                let cost = product.price_per_unit * quantity;
                println!(
                    "- {} {} {}, price: {:.2} {}",
                    quantity, unit, product_name, cost, product_unit_parts[0]
                );
                total_cost += cost;
            } else {
                println!(
                    "Unit for '{}' does not match the stored unit '{}'. Skipping item.",
                    product_name, product.unit
                );
            }
        } else {
            println!("Product '{}' not found in the data.", product_name);
        }
    }

    println!("\nTotal price: {:.2} UAH", total_cost);
}
