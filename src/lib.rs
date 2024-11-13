//! # Grammar Rules Documentation
//!
//! ## Rules Overview
//!
//! - **WHITESPACE** - matches spaces or tabs.
//! - **product_name** - matches a product name in the format `Product name: <name>`.
//! - **category** - matches a category in the format `Category: <category>`.
//! - **price** - matches a price in the format `Price: <amount> <currency>/<unit>`.
//! - **calories** - matches the caloric content in the format `Calories: <calories> cal`.
//! - **proteins** - matches the protein content in grams in the format `Proteins: <amount> g`.
//! - **carbohydrates** - matches the carbohydrate content in grams in the format `Carbohydrates: <amount> g`.
//! - **fats** - matches the fat content in grams in the format `Fats: <amount> g`.
//! - **currency_amount** - matches an amount with an optional decimal part.
//! - **currency** - supported currencies: `UAH`, `USD`, `EUR`.
//! - **unit** - supported units: `kg`, `l`, `ml`, `pcs`, `g`.
//! - **name** - represents the name of a product, consisting of alphabetic characters and allowing multiple words separated by spaces.
//! - **product** - matches a single product entry with all properties.
//! - **products** - matches a list of products, each separated by a blank line.
//! - **shopping_item** - matches a single shopping item in the format `<name> <quantity> <unit>`.
//! - **shopping_list** - matches a shopping list with multiple items separated by commas.

use anyhow::anyhow;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    pub product_name: String,
    pub category: String,
    pub price_per_unit: f64,
    pub unit: String,
    pub calories: f64,
    pub proteins: f64,
    pub carbohydrates: f64,
    pub fats: f64,
}

#[derive(Debug)]
pub struct ShoppingItem {
    pub product_name: String,
    pub quantity: f64,
    pub unit: String,
}

impl Product {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        let mut product_name = String::new();
        let mut category = String::new();
        let mut price_per_unit = 0.0;
        let mut unit = String::new();
        let mut calories = 0.0;
        let mut proteins = 0.0;
        let mut carbohydrates = 0.0;
        let mut fats = 0.0;

        for field in pair.into_inner() {
            match field.as_rule() {
                Rule::product_name => {
                    product_name = field
                        .into_inner()
                        .find(|inner| inner.as_rule() == Rule::name)
                        .map(|name| name.as_str().to_string())
                        .unwrap_or_default();
                }
                Rule::category => {
                    category = field.as_str().replace("Category:", "").trim().to_string();
                }
                Rule::price => {
                    for inner in field.into_inner() {
                        match inner.as_rule() {
                            Rule::currency_amount => {
                                price_per_unit =
                                    inner.as_str().trim().parse::<f64>().unwrap_or(0.0);
                            }
                            Rule::currency => {
                                unit = inner.as_str().to_string();
                            }
                            Rule::unit => {
                                unit.push('/');
                                unit.push_str(inner.as_str());
                            }
                            _ => {}
                        }
                    }
                }
                Rule::calories => {
                    let calories_str = field
                        .as_str()
                        .replace("Calories:", "")
                        .replace("cal", "")
                        .trim()
                        .to_string();
                    calories = calories_str.parse::<f64>().unwrap_or(0.0);
                }
                Rule::proteins => {
                    let proteins_str = field
                        .as_str()
                        .replace("Proteins:", "")
                        .replace("g", "")
                        .trim()
                        .to_string();
                    proteins = proteins_str.parse::<f64>().unwrap_or(0.0);
                }
                Rule::carbohydrates => {
                    let carbohydrates_str = field
                        .as_str()
                        .replace("Carbohydrates:", "")
                        .replace("g", "")
                        .trim()
                        .to_string();
                    carbohydrates = carbohydrates_str.parse::<f64>().unwrap_or(0.0);
                }
                Rule::fats => {
                    let fats_str = field
                        .as_str()
                        .replace("Fats:", "")
                        .replace("g", "")
                        .trim()
                        .to_string();
                    fats = fats_str.parse::<f64>().unwrap_or(0.0);
                }
                _ => {}
            }
        }

        Product {
            product_name,
            category,
            price_per_unit,
            unit,
            calories,
            proteins,
            carbohydrates,
            fats,
        }
    }
}

pub fn parse_shopping_list(input: &str) -> Result<Vec<ShoppingItem>, anyhow::Error> {
    let pairs = Grammar::parse(Rule::shopping_list, input)
        .map_err(|e| anyhow!("Parsing shopping list failed: {}", e))?;

    let mut items = Vec::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::shopping_item => {
                let mut product_name = String::new();
                let mut quantity = 0.0;
                let mut unit = String::new();

                for field in pair.into_inner() {
                    match field.as_rule() {
                        Rule::name => {
                            product_name = field.as_str().to_string();
                        }
                        Rule::currency_amount => {
                            quantity = field.as_str().parse::<f64>().unwrap_or(0.0);
                        }
                        Rule::unit => {
                            unit = field.as_str().to_string();
                        }
                        _ => {}
                    }
                }

                items.push(ShoppingItem {
                    product_name,
                    quantity,
                    unit,
                });
            }
            _ => {}
        }
    }

    Ok(items)
}



pub fn parse_products_file(file_path: &str) -> Result<Vec<Product>, anyhow::Error> {
    let content = fs::read_to_string(file_path)?;
    let pairs = Grammar::parse(Rule::products, &content)
        .map_err(|e| anyhow!("Parsing failed: {}", e))?
        .next()
        .ok_or_else(|| anyhow!("No products found in input file"))?;

    let products: Vec<Product> = pairs
        .into_inner()
        .filter_map(|pair| {
            if pair.as_rule() == Rule::product {
                Some(Product::from_pair(pair))
            } else {
                None
            }
        })
        .collect();

    Ok(products)
}

pub fn save_products_to_json(products: &[Product], output_path: &str) -> io::Result<()> {
    let json_data = serde_json::to_string_pretty(products)?;
    let mut file = File::create(output_path)?;
    file.write_all(json_data.as_bytes())?;
    Ok(())
}

pub fn load_products_from_json(input_path: &str) -> Result<Vec<Product>, io::Error> {
    let mut file = File::open(input_path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let products: Vec<Product> = serde_json::from_str(&data)?;
    Ok(products)
}

