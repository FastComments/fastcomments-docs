[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

По умолчанию FastComments будет отображать бейджи пользователей только в их комментариях внутри потока комментариев.

Однако мы можем показать бейджи пользователя рядом с их именем над формой комментария, включив эту функцию на странице настройки виджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; title='Show Badges in Top Bar Option' app-screenshot-end]

Это отображает бейджи пользователя рядом с его именем в верхней панели, делая достижения и статус более заметными во время написания комментария.

Учтите, что эта функция должна быть включена в интерфейсе настройки виджета, чтобы работать. Вы можете при желании установить флаг **showBadgesInTopBar** в false в конфигурации кода, чтобы выборочно отключить его даже когда он включен на стороне сервера:

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Disable Show Badges in Top Bar'; code-example-end]