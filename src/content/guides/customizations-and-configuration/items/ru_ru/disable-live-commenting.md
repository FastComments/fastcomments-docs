[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

По умолчанию FastComments будет иметь включённые живые комментарии.

Это означает, что каждый зритель ветки комментариев будет видеть одинаковый контент.

Например, если добавлен комментарий, он должен отображаться. Если комментарий отредактирован или удалён,
то эти комментарии будут отредактированы или удалены для всех зрителей ветки. То же самое относится к голосованиям и всем действиям модерации.

Однако мы можем отключить это:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Отключить живые комментарии'; code-example-end]

Это также можно сделать без кода. На странице настройки виджета см. раздел «Отключить живые комментарии».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; alt='Раздел Отключить живые комментарии на странице настройки виджета, отключающий обновления ветки в реальном времени'; title='Отключить живые комментарии' app-screenshot-end]