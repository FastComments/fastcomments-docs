[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

За замовчуванням FastComments дозволяє користувачеві вводити коментар у будь‑якій кількості рядків, до досягнення стандартного ліміту символів.

Однак може бути бажаним обмежити користувача вводом лише одного рядка тексту. Прикладами використання можуть бути онлайн‑аукціони або живий чат, для яких можна використовувати FastComments.

Ми вмикаємо прапорець **useSingleLineCommentInput** наступним чином:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Увімкнути однорядковий ввід коментаря'; code-example-end]

Це також можна зробити без коду. На сторінці налаштування віджету перегляньте розділ "Увімкнути однорядковий ввід коментаря".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; alt='Прапорець вводу однорядкового коментаря увімкнено на сторінці налаштування віджету, обмежуючи ввід одним рядком'; title='Увімкнути однорядковий ввід коментаря' app-screenshot-end]

Зверніть увагу, що коментарі на кожній сторінці для кожного напрямку сортування попередньо обчислюються, тому всі напрямки сортування мають однакову продуктивність.