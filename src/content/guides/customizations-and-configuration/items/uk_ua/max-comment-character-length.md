[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

Максимальну кількість символів, які допускається вводити в полі коментаря, можна обмежити параметром **maxCommentCharacterLength**.

Значення за замовчуванням — 2000.

Такі елементи, як URL-адреси зображень, не враховуються при визначенні довжини.

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = 'Limit Comment Length'; code-example-end]

Це можна налаштувати без коду на сторінці налаштування віджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; title='Limit Comment Length' app-screenshot-end]

---