[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

По умолчанию FastComments позволяет пользователям блокировать других пользователей. Блокировка пользователя приведёт к маскированию их комментариев, предотвратит уведомления между пользователями и т.д.

Возможно, потребуется отключить эту функцию. Это можно сделать следующим образом:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Отключить блокировку'; code-example-end]

Это также можно сделать без кода, что обеспечивает правильную серверную проверку, через пользовательский интерфейс настройки виджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; alt='Опция отключения блокировки в пользовательском интерфейсе настройки виджета, которая предотвращает блокировку пользователей друг другом'; title='Отключить блокировку' app-screenshot-end]