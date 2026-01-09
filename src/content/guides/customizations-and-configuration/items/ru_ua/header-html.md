[related-parameter-start name = 'headerHTML'; type = 'string'; related-parameter-end]

Некоторый текст, например заголовок или сообщение, может отображаться ниже счётчика комментариев, но выше текста статуса входа.

Мы называем это заголовком, и по умолчанию он скрыт.

[code-example-start config = {headerHTML: "<h1>Leave a Comment!</h1>"}; linesToHighlight = [6]; title = 'Specifying Header HTML'; code-example-end]

Это можно настроить без кода на странице настройки виджета, в разделе «Дополнительные параметры»:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.absolute-dates'; title='Specifying Header HTML' app-screenshot-end]