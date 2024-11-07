use shopping_parser::parser::parse_shopping_item;
use shopping_parser::data::ShoppingItem;
#[test]
//
fn test_parse_shopping_item() {
    let input = "яблука 2 кг - фрукти";
    let expected = ShoppingItem {
        item: "яблука".to_string(),
        quantity: 2.0,
        unit: "кг".to_string(),
        category: "фрукти".to_string(),
    };

    let result = parse_shopping_item(input).unwrap();
    assert_eq!(result, expected);
}

