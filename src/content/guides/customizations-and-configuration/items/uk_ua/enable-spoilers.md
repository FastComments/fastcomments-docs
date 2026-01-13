[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

Ми можемо увімкнути підтримку спойлерів, встановивши прапорець **enableSpoilers** в true:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Enabling Spoilers'; code-example-end]

Це також можна зробити без коду. На сторінці налаштування віджета знайдіть опцію "Enable Spoilers".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; title='Enable Spoilers' app-screenshot-end]

Коли текст виділяють і натискають тепер видиму кнопку `SPOILER`, текст буде замасковано, поки користувач не наведе на нього курсор. У темному режимі ми робимо те ж саме, але з іншими
кольорами, які краще підходять до темного режиму.

Це також сумісно з WYSIWYG-редактором.