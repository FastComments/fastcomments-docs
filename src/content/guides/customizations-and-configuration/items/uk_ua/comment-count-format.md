[related-parameter-start name = 'commentCountFormat'; type = 'string'; related-parameter-end]

Кількість коментарів, що відображається у верхній частині віджета коментарів, можна налаштувати.

Це можна замінити будь‑яким рядком, і значення **[count]** буде замінено на кількість, локалізовану для користувача.

[code-example-start config = {commentCountFormat: "There are [count] comments."}; linesToHighlight = [6]; title = 'Налаштування тексту кількості коментарів'; code-example-end]

Це можна налаштувати без коду на сторінці налаштування віджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.comment-count'; alt='Поле тексту кількості коментарів на сторінці налаштування віджета, де [count] замінюється на поточну загальну кількість'; title='Налаштування тексту кількості коментарів' app-screenshot-end]