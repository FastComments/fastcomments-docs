---
[related-parameter-start name = 'startingPage'; type = 'number'; related-parameter-end]

Під час отримання та відображення коментарів віджет коментарів має знати, з якої сторінки починати. За замовчуванням він починається з
першої сторінки і відображає лише цю сторінку.

Якщо потрібно, точну сторінку, яку слід відобразити, можна передати віджету коментарів через налаштування *startingPage*.

[code-example-start config = {startingPage: 1}; linesToHighlight = [6]; title = 'Specifying The Page to Render'; code-example-end]

Зауважте, що нумерація сторінок починається з нуля, тому вищенаведений приклад відображає другу сторінку.

---