[related-parameter-start name = 'countAll'; type = 'boolean'; related-parameter-end]

Кількість коментарів, яка відображається у верхній частині віджета коментарів, може показувати або всі "top-level" коментарі, тобто ті відповіді, що
є відповідями безпосередньо до сторінки або статті, або вона може бути підрахунком **усіх** вкладених коментарів.

За замовчуванням це `true` - це підрахунок останнього варіанту - усіх коментарів. У старіших версіях віджета коментарів значення за замовчуванням було `false`.

Ми можемо змінити поведінку так, щоб вона була підрахунком **усіх** вкладених коментарів, встановивши прапорець **countAll** в true.

[code-example-start config = {countAll: true}; linesToHighlight = [6]; title = 'Counting All Comments'; code-example-end]

Якщо ми хочемо, щоб підрахунок відображав лише коментарі верхнього рівня, ми встановлюємо прапорець у false.

[code-example-start config = {countAll: false}; linesToHighlight = [6]; title = 'Counting Top Level Comments'; code-example-end]

Наразі це не можна налаштувати без змін у коді.