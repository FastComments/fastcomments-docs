[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

Максимальна кількість символів, які можна ввести в поле вводу коментаря, може бути обмежена параметром **maxCommentCharacterLength**.

За замовчуванням це 2000.

Такі речі, як URL‑зображень, не включаються у визначення довжини.

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = 'Обмежити довжину коментаря'; code-example-end]

Це можна налаштувати без коду на сторінці налаштування віджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; alt='Поле розміру максимального коментаря на сторінці налаштування віджета, використовується для обмеження кількості символів у коментарі'; title='Обмежити довжину коментаря' app-screenshot-end]

---