# Shopping Parser

## https://crates.io/crates/shopping-parser 
## https://docs.rs/shopping-parser/latest/shopping_parser/

## Project Overview

Shopping Parser is a Rust-based command-line application that parses structured product information and shopping lists, with support for multiple units (e.g., kg, liters, grams) and currencies (e.g., UAH, USD, EUR). The application is designed for users who want to store detailed product information in a structured JSON format and calculate total costs based on shopping lists.

## Features

**Parse Product Information**: The program reads structured product data from a text file and saves it in JSON format. This data includes:
   - Product name and category
   - Price per unit in different currencies and units
   - Calorie, protein, carbohydrate, and fat content

**Retrieve Product Information**: Users can search for specific products by name, displaying all details stored in the JSON format.

**Calculate Shopping List Total**: Users can input a shopping list of items with quantities. The program calculates the total cost based on the stored JSON data and supports different units and currencies.

**Error Handling**: Built-in error handling is implemented using `anyhow` for comprehensive error reporting during command execution and `thiserror` for library-level error handling.

## CLI Commands

The program includes the following commands:

- **Parse Product File**: `--parse <input_file> <output_file>`
Reads structured product data from a text file and saves it in JSON format for future retrieval and shopping list calculations.

- **Retrieve Product Data**: `--get <product_name>`
Displays detailed information about a specified product from the stored JSON data.

- **Calculate Total from Shopping List**: `--list <shopping_list>`
Calculates the total cost based on a comma-separated list of products and quantities. It will match product names and units to stored data and calculate the price accordingly.

- **Credits**: `--credits`
Displays credits and developer information.

- **Help**: `--help`
Displays information on available commands.

## Parsing Details

### Example Product File Parsing

Below is an example of a text file containing product data, structured in the required format:

```
Product name: apple
Category: fruit
Price: 10 UAH/kg
Calories: 52 cal
Proteins: 0.3 g
Carbohydrates: 14 g
Fats: 0.2 g

Product name: milk
Category: dairy
Price: 20 UAH/l
Calories: 42 cal
Proteins: 3.4 g
Carbohydrates: 5 g
Fats: 3.2 g
```

To parse this file and save the data to a JSON file, use the following command:
```
cargo run -- --parse input.txt output.json
```
The parsed data will be saved in output.json.

### Example Shopping List Command

To calculate the total cost for a shopping list, use the --list command with the list of items as shown below:
```
cargo run -- --list "apple 2 kg, milk 1 l"
```
The expected output will display the total price for each item and the combined total:
```
- 2 kg apple, price: 20.00 UAH
- 1 l milk, price: 20.00 UAH

Total price: 40.00 UAH
```

## Grammar overview
```
// WHITESPACE - matches spaces or tabs.
WHITESPACE = { " " | "\t" }

// currency - represents supported currencies: "UAH", "USD", and "EUR".
currency = { "UAH" | "USD" | "EUR" }

// unit - defines supported measurement units: "kg", "l", "ml", "pcs", "g".
unit = { "kg" | "l" | "ml" | "pcs" | "g" }

// name - represents the name of a product, consisting of alphabetic characters and allowing multiple words separated by spaces.
name = { ASCII_ALPHA+ ~ (WHITESPACE+ ~ ASCII_ALPHA+)* }

// product_name - matches the product name in the format "Product name: <name>".
product_name = { ("Product name:" ~ WHITESPACE*)? ~ name }

// category - matches the product category in the format "Category: <category>".
category = { "Category:" ~ WHITESPACE* ~ ASCII_ALPHANUMERIC+ }

// price - matches the product price in the format "Price: <amount> <currency>/<unit>".
price = { "Price:" ~ WHITESPACE* ~ currency_amount ~ WHITESPACE* ~ currency ~ "/" ~ unit }

// calories - matches the calorie content in the format "Calories: <calories> cal".
calories = { "Calories:" ~ WHITESPACE* ~ ASCII_DIGIT+ ~ WHITESPACE* ~ "cal" }

// proteins - matches the protein content in grams in the format "Proteins: <amount> g".
proteins = { "Proteins:" ~ WHITESPACE* ~ ASCII_DIGIT+ ~ ("." ~ ASCII_DIGIT+)? ~ WHITESPACE* ~ "g" }

// carbohydrates - matches the carbohydrate content in grams in the format "Carbohydrates: <amount> g".
carbohydrates = { "Carbohydrates:" ~ WHITESPACE* ~ ASCII_DIGIT+ ~ ("." ~ ASCII_DIGIT+)? ~ WHITESPACE* ~ "g" }

// fats - matches the fat content in grams in the format "Fats: <amount> g".
fats = { "Fats:" ~ WHITESPACE* ~ ASCII_DIGIT+ ~ ("." ~ ASCII_DIGIT+)? ~ WHITESPACE* ~ "g" }

// currency_amount - represents a numeric value with an optional decimal part.
currency_amount = { ASCII_DIGIT+ ~ ("." ~ ASCII_DIGIT+)? }

// product - defines a complete product entry with all fields: name, category, price, calories, proteins, carbohydrates, fats.
product = {
    product_name ~ NEWLINE ~
    category ~ NEWLINE ~
    price ~ NEWLINE ~
    calories ~ NEWLINE ~
    proteins ~ NEWLINE ~
    carbohydrates ~ NEWLINE ~
    fats
}

// products - represents a list of products, where each product entry is separated by a blank line.
products = { product ~ (NEWLINE ~ NEWLINE ~ product)+ ~ NEWLINE* }

// shopping_item - represents a single item in the shopping list, following the format "<name> <quantity> <unit>".
shopping_item = { name ~ WHITESPACE+ ~ currency_amount ~ WHITESPACE+ ~ unit }

// shopping_list - represents a shopping list with multiple items, separated by commas.
shopping_list = { shopping_item ~ (WHITESPACE* ~ "," ~ WHITESPACE* ~ shopping_item)* }
```
