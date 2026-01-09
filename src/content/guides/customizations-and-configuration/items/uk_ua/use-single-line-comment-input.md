---
[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

За замовчуванням FastComments дозволяє користувачеві вводити коментар з будь-якою кількістю рядків, до стандартного обмеження на кількість символів.

Однак може виникнути потреба обмежити введення лише одним рядком тексту. Прикладами таких сценаріїв є онлайн-торги або чат в реальному часі, для яких FastComments
можна використовувати.

Ми вмикаємо прапорець **useSingleLineCommentInput** таким чином:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Enable Single-Line Comment Input'; code-example-end]

Це також можна зробити без коду. На сторінці налаштування віджета див. розділ "Увімкнути введення коментаря в один рядок".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; title='Enable Single-Line Comment Input' app-screenshot-end]

Зауважте, що коментарі на кожній сторінці для кожного напрямку сортування попередньо обчислені, тому всі напрямки сортування мають однакову продуктивність.

---