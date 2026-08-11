[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

Ми можемо ввімкнути підтримку спойлерів, встановивши прапорець **enableSpoilers** у значення true:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Увімкнення спойлерів'; code-example-end]

Це також можна зробити без коду. На сторінці налаштування віджету, перегляньте опцію "Enable Spoilers".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; alt='Сторінка налаштування віджету з позначеним прапорцем Enable Spoilers, щоб додати кнопку SPOILER до редактора'; title='Увімкнути спойлери' app-screenshot-end]

Коли текст виділено, і тепер видима кнопка `SPOILER` натискається, текст буде замасковано, доки користувач не наведеться на нього мишкою. Для темного режиму ми робимо те ж саме, з іншими кольорами, які краще підходять для темного режиму.

Це також сумісно з редактором WYSIWYG.