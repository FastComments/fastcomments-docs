[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

По умолчанию FastComments отображает значки пользователей только на их комментариях в ветке комментариев.

Однако мы можем показывать значки пользователей рядом с их именем над формой комментария, включив эту функцию на странице настройки виджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; title='Show Badges in Top Bar Option' app-screenshot-end]

Это будет отображать значки пользователя рядом с его именем в верхней панели, делая их достижения и статус более заметными, когда пользователь пишет комментарий.

Обратите внимание, что эта функция должна быть включена в интерфейсе настройки виджета, чтобы работать. При необходимости вы можете установить флаг **showBadgesInTopBar** в false в конфигурации кода, чтобы выборочно отключить её, даже если он включён на уровне сервера:

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Disable Show Badges in Top Bar'; code-example-end]
---