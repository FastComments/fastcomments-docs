[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

По умолчанию FastComments позволяет пользователям блокировать других пользователей. Блокировка пользователя приведёт к маскировке его комментариев
и предотвратит отправку уведомлений между пользователями и т.д.

Иногда может потребоваться отключить эту функцию. Это можно сделать следующим образом:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Disable Blocking'; code-example-end]

Это также можно сделать без кода, что дополнительно обеспечивает корректную проверку на стороне сервера, через интерфейс настройки виджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; title='Disable Blocking' app-screenshot-end]

---