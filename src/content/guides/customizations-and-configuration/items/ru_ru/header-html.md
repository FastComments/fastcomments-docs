[related-parameter-start name = 'headerHTML'; type = 'string'; related-parameter-end]

Некоторый текст, например заголовок или сообщение, может отображаться ниже количества комментариев, но выше текста статуса входа.

Мы называем это заголовком, и по умолчанию он скрыт.

[code-example-start config = {headerHTML: "<h1>Leave a Comment!</h1>"}; linesToHighlight = [6]; title = 'Specifying Header HTML'; code-example-end]

Это можно настроить без кода на странице настройки виджета, в разделе Расширенные параметры:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.absolute-dates'; alt='Область Расширенных параметров на странице настройки виджета, где вводится пользовательский HTML заголовка'; title='Указание HTML заголовка' app-screenshot-end]