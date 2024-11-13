use shopping_parser::Rule;

#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;
    use shopping_parser::{Grammar, Rule};

    #[test]
    fn test_product_name() {
        let pairs = Grammar::parse(Rule::product_name, "Product name: apple").unwrap();
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
            assert!(pairs.into_iter().next().is_some());
        }
    }

    #[test]
    fn test_calories() {
        let pairs = Grammar::parse(Rule::calories, "Calories: 52 cal").unwrap();
        assert!(pairs.into_iter().next().is_some());
    }

    #[test]
    fn test_proteins() {
        let pairs = Grammar::parse(Rule::proteins, "Proteins: 0.3 g").unwrap();
        assert!(pairs.into_iter().next().is_some());
    }

    #[test]
    fn test_carbohydrates() {
        let pairs = Grammar::parse(Rule::carbohydrates, "Carbohydrates: 9.42 g").unwrap();
        assert!(pairs.into_iter().next().is_some());
    }

    #[test]
    fn test_fats() {
        let pairs = Grammar::parse(Rule::fats, "Fats: 0.2 g").unwrap();
        assert!(pairs.into_iter().next().is_some());
    }

    #[test]
    fn test_currency_amount() {
        let valid_amounts = vec!["10", "5.5", "100.75"];
        for amount in valid_amounts {
            let pairs = Grammar::parse(Rule::currency_amount, amount).unwrap();
            assert!(pairs.into_iter().next().is_some());
        }
    }

    #[test]
    fn test_currency() {
        let valid_currencies = vec!["UAH", "USD", "EUR"];
        for currency in valid_currencies {
            let pairs = Grammar::parse(Rule::currency, currency).unwrap();
            assert!(pairs.into_iter().next().is_some());
        }
    }

    #[test]
    fn test_unit() {
        let valid_units = vec!["kg", "l", "ml", "pcs", "g"];
        for unit in valid_units {
            let pairs = Grammar::parse(Rule::unit, unit).unwrap();
            assert!(pairs.into_iter().next().is_some());
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
        assert!(pairs.into_iter().next().is_some());
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
        assert!(pairs.into_iter().next().is_some());
    }
}
