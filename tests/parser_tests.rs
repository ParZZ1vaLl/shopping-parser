#[cfg(test)]
mod tests {
    use anyhow::Result;
    use pest::Parser;
    use shopping_parser::{parse_shopping_list, Grammar, Rule, ShoppingItem};

    #[test]
    fn test_whitespace() {
        // Тестуємо правило WHITESPACE
        let valid_whitespaces = vec![" ", "\t", "  ", "\t\t", " \t "];
        for ws in valid_whitespaces {
            let pairs = Grammar::parse(Rule::WHITESPACE, ws).unwrap();
            assert!(
                pairs.into_iter().next().is_some(),
                "Failed to parse whitespace: '{}'",
                ws
            );
        }
    }

    #[test]
    fn test_currency() {
        let valid_currencies = vec!["UAH", "USD", "EUR"];
        for currency in valid_currencies {
            let pairs = Grammar::parse(Rule::currency, currency).unwrap();
            assert!(
                pairs.into_iter().next().is_some(),
                "Failed to parse currency: '{}'",
                currency
            );
        }
    }

    #[test]
    fn test_unit() {
        let valid_units = vec!["kg", "l", "ml", "pcs", "g"];
        for unit in valid_units {
            let pairs = Grammar::parse(Rule::unit, unit).unwrap();
            assert!(
                pairs.into_iter().next().is_some(),
                "Failed to parse unit: '{}'",
                unit
            );
        }
    }

    #[test]
    fn test_name() {
        let valid_names = vec!["apple", "green apple", "red delicious apple"];
        for name in valid_names {
            let pairs = Grammar::parse(Rule::name, name).unwrap();
            assert!(
                pairs.into_iter().next().is_some(),
                "Failed to parse name: '{}'",
                name
            );
        }
    }

    #[test]
    fn test_product_name() {
        let pairs = Grammar::parse(Rule::product_name, "Product name: apple").unwrap();
        assert!(pairs.into_iter().next().is_some());

        // Тест з опціональним "Product name:"
        let pairs = Grammar::parse(Rule::product_name, "apple").unwrap();
        assert!(pairs.into_iter().next().is_some());
    }

    #[test]
    fn test_category() {
        let pairs = Grammar::parse(Rule::category, "Category: fruit").unwrap();
        assert!(pairs.into_iter().next().is_some());
    }

    #[test]
    fn test_price() {
        let valid_prices = vec!["Price: 10 UAH/kg", "Price: 5.5 USD/l", "Price: 20 EUR/ml"];
        for price in valid_prices {
            let pairs = Grammar::parse(Rule::price, price).unwrap();
            assert!(
                pairs.into_iter().next().is_some(),
                "Failed to parse price: '{}'",
                price
            );
        }
    }

    #[test]
    fn test_calories() {
        let pairs = Grammar::parse(Rule::calories, "Calories: 52 cal").unwrap();
        assert!(pairs.into_iter().next().is_some());
    }

    #[test]
    fn test_proteins() {
        let valid_proteins = vec!["Proteins: 0.3 g", "Proteins: 5 g"];
        for protein in valid_proteins {
            let pairs = Grammar::parse(Rule::proteins, protein).unwrap();
            assert!(
                pairs.into_iter().next().is_some(),
                "Failed to parse proteins: '{}'",
                protein
            );
        }
    }

    #[test]
    fn test_carbohydrates() {
        let valid_carbs = vec!["Carbohydrates: 9.42 g", "Carbohydrates: 15 g"];
        for carb in valid_carbs {
            let pairs = Grammar::parse(Rule::carbohydrates, carb).unwrap();
            assert!(
                pairs.into_iter().next().is_some(),
                "Failed to parse carbohydrates: '{}'",
                carb
            );
        }
    }

    #[test]
    fn test_fats() {
        let valid_fats = vec!["Fats: 0.2 g", "Fats: 1.5 g"];
        for fat in valid_fats {
            let pairs = Grammar::parse(Rule::fats, fat).unwrap();
            assert!(
                pairs.into_iter().next().is_some(),
                "Failed to parse fats: '{}'",
                fat
            );
        }
    }

    #[test]
    fn test_currency_amount() {
        let valid_amounts = vec!["10", "5.5", "100.75"];
        for amount in valid_amounts {
            let pairs = Grammar::parse(Rule::currency_amount, amount).unwrap();
            assert!(
                pairs.into_iter().next().is_some(),
                "Failed to parse currency amount: '{}'",
                amount
            );
        }
    }

    #[test]
    fn test_product() {
        let input = "\
Product name: apple
Category: fruit
Price: 10 UAH/kg
Calories: 52 cal
Proteins: 0.3 g
Carbohydrates: 9.42 g
Fats: 0.2 g";

        let pairs = Grammar::parse(Rule::product, input).unwrap();
        assert!(
            pairs.into_iter().next().is_some(),
            "Failed to parse product:\n{}",
            input
        );
    }

    #[test]
    fn test_products() {
        let input = "\
Product name: apple
Category: fruit
Price: 10 UAH/kg
Calories: 52 cal
Proteins: 0.3 g
Carbohydrates: 9.42 g
Fats: 0.2 g

Product name: milk
Category: dairy
Price: 15 USD/l
Calories: 42 cal
Proteins: 3.6 g
Carbohydrates: 4.8 g
Fats: 3.6 g";

        let pairs = Grammar::parse(Rule::products, input).unwrap();
        assert!(
            pairs.into_iter().next().is_some(),
            "Failed to parse products:\n{}",
            input
        );
    }

    #[test]
    fn test_shopping_item() {
        let input = "apple 2 kg";
        let result = parse_shopping_list(input);
        assert!(
            result.is_ok(),
            "Parsing failed for test_shopping_item: {:?}",
            result.unwrap_err()
        );

        let items = result.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].product_name, "apple");
        assert_eq!(items[0].quantity, 2.0);
        assert_eq!(items[0].unit, "kg");
    }

    #[test]
    fn test_shopping_list() {
        let input = "apple 2 kg, milk 1 l, chicken 0.5 kg";
        let result = parse_shopping_list(input);
        assert!(
            result.is_ok(),
            "Parsing failed for test_shopping_list: {:?}",
            result.unwrap_err()
        );

        let items = result.unwrap();
        assert_eq!(items.len(), 3);

        assert_eq!(items[0].product_name, "apple");
        assert_eq!(items[0].quantity, 2.0);
        assert_eq!(items[0].unit, "kg");

        assert_eq!(items[1].product_name, "milk");
        assert_eq!(items[1].quantity, 1.0);
        assert_eq!(items[1].unit, "l");

        assert_eq!(items[2].product_name, "chicken");
        assert_eq!(items[2].quantity, 0.5);
        assert_eq!(items[2].unit, "kg");
    }
}
