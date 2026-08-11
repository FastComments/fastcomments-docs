[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

За замовчуванням FastComments запитує у користувача лише їхній коментар, ім'я користувача та електронну пошту.

Однак у деяких ситуаціях ви можете захотіти, щоб користувач залишив посилання на свій блог або веб‑сайт.

Ми можемо ввімкнути показ додаткового поля вводу для залишення URL веб‑сайту користувача, встановивши прапорець **enableCommenterLinks** у true:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Увімкнення посилань коментатора'; code-example-end]

Коли вказаний URL надається, обліковий запис користувача буде оновлено, і всі їхні імена користувачів у всіх минулих та майбутніх коментарях будуть посиланнями на цей URL.

Це можна налаштувати без коду на сторінці налаштування віджету:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; alt='Сторінка налаштування віджету з позначеним прапорцем посилань коментатора, щоб додати поле URL веб‑сайту до форми коментаря'; title='Увімкнення посилань коментатора' app-screenshot-end]