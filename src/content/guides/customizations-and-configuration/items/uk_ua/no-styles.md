[related-parameter-start name = 'noStyles'; type = 'boolean'; related-parameter-end]

Для великих проєктів кастомного стилювання може бути бажаним почати з чистого листа і не використовувати стилі за замовчуванням.

Усі стилі за замовчуванням можна видалити, встановивши параметр **noStyles** у значення true, наступним чином:

[code-example-start config = {noStyles: true}; linesToHighlight = [6]; title = 'Disabling All Default Styles'; code-example-end]

Це можна налаштувати без коду на сторінці налаштування віджету в розділі «Розширені параметри»:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.disable-all-default-styling']; selector = '.disable-all-default-styling'; alt='Прапорець «Вимкнути всі стилі за замовчуванням», увімкнений у розділі «Розширені параметри» на сторінці налаштування віджету'; title='Вимкнення всіх стилів за замовчуванням' app-screenshot-end]