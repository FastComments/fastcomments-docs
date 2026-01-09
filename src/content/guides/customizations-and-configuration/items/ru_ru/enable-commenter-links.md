[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

По умолчанию FastComments будет запрашивать у пользователя только сам комментарий, имя пользователя и адрес электронной почты.

Однако в некоторых случаях вы можете захотеть, чтобы пользователь оставил ссылку на свой блог или сайт.

Мы можем включить отображение дополнительного поля ввода для указания URL сайта пользователя, установив флаг **enableCommenterLinks** в true:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Включение ссылок комментаторов'; code-example-end]

Если такой URL указан, аккаунт пользователя будет обновлён, и все его имена пользователей во всех прошлых и будущих комментариях будут ссылаться на этот URL.

Это можно настроить без кода на странице настройки виджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; title='Включение ссылок комментаторов' app-screenshot-end]

---