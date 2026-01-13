[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

За замовчуванням FastComments проситиме користувача лише залишити коментар, ім'я користувача та електронну пошту.

Однак у деяких випадках ви можете захотіти, щоб користувач залишив посилання на власний блог або вебсайт.

Ми можемо ввімкнути відображення додаткового поля вводу для URL вебсайту користувача, встановивши прапорець **enableCommenterLinks** у значення true:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Enabling Commenter Links'; code-example-end]

Якщо такий URL буде вказано, обліковий запис користувача оновиться, і всі його імена користувачів у минулих та майбутніх коментарях будуть містити посилання на цей URL.

Це можна налаштувати без коду на сторінці налаштування віджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; title='Enabling Commenter Links' app-screenshot-end]

---