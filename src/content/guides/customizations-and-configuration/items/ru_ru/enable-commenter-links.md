[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

По умолчанию FastComments запрашивает у пользователя только их комментарий, имя пользователя и электронную почту.

Однако в некоторых ситуациях вы можете захотеть, чтобы пользователь оставил ссылку на свой блог или веб‑сайт.

Мы можем включить отображение дополнительного поля ввода для указания URL веб‑сайта пользователя, установив флаг **enableCommenterLinks** в значение true:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Включение ссылок комментатора'; code-example-end]

Когда указанный URL предоставлен, учетная запись пользователя будет обновлена, и все их имена пользователей во всех прошлых и будущих комментариях будут ссылаться на этот URL.

Это можно настроить без кода на странице настройки виджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; alt='Страница настройки виджета с отмеченным флажком ссылок комментатора для добавления поля URL веб‑сайта в форму комментария'; title='Включение ссылок комментатора' app-screenshot-end]