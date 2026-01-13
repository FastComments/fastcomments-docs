[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

По умолчанию FastComments отображает варианты голосования в виде стрелок вверх и вниз, позволяя пользователям либо голосовать за комментарий, либо против него.

Однако можно изменить стиль панели голосования. Текущие варианты — стандартные кнопки Up/Down или механизм голосования в стиле Heart.

Мы используем флаг **voteStyle** следующим образом:

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Enable Heart Button'; code-example-end]

Мы настоятельно рекомендуем делать это без кода, так как это также включает серверную валидацию. На странице настройки виджета смотрите раздел "Vote Style".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; title='Change Voting Style' app-screenshot-end]

Голосование также можно отключить, см. `Disable Voting` выше параметров стиля.

---