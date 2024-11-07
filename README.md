# shopping-parser

## Загальний опис проєкту
**shopping-parser** — це консольний застосунок, який допомагає зі структуризацією простих списків покупок. Він розпізнає різні категорії товарів, кількість та одиниці вимірювання. Метою проєкту є допомогти користувачам впорядковувати свої списки покупок, сортувати товари за категоріями та видами одиниць вимірювання.

## Як це працює?
**shopping-parser** використовує просту граматику для розпізнавання елементів у списку покупок:
- **Назва товару (item)** — основний предмет у списку.
- **Кількість (quantity)** — число, яке вказує на кількість або обсяг товару.
- **Одиниця вимірювання (unit)** — визначає у яких одиницях вимірюється товар (наприклад, "кг", "л", "шт").
- **Категорія (category)** — класифікація товару за типом (наприклад, "фрукти", "молочні продукти", "випічка").

Програма аналізує вхідний текст, розбиває його на компоненти та структурує їх у зручному форматі. Це дозволяє користувачам переглядати список за категоріями або одиницями вимірювання, що полегшує планування покупок.

## Приклад
Формат запису списку покупок:

> яблука 2 кг - фрукти, молоко 1 л - молочні продукти, яйця 12 шт - бакалія, хліб 1 шт - випічка

Після парсингу, програма структурує дані і дозволяє переглядати список за категоріями чи одиницями вимірювання.


