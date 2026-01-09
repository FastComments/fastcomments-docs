[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

По умолчанию FastComments запрашивает у пользователя только его комментарий, имя пользователя и адрес электронной почты.

Однако в некоторых случаях вы можете захотеть, чтобы пользователь оставил ссылку на свой блог или веб-сайт.

Мы можем включить показ дополнительного поля ввода для URL сайта пользователя, установив флаг **enableCommenterLinks** в true:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Enabling Commenter Links'; code-example-end]

Когда такой URL указан, аккаунт пользователя будет обновлён, и отображаемые имена пользователя во всех предыдущих и последующих комментариях будут ссылаться на этот URL.

Это можно настроить без кода на странице настройки виджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; title='Enabling Commenter Links' app-screenshot-end]