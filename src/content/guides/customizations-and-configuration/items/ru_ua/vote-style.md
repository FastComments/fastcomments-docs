[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

По умолчанию FastComments отображает варианты голосования в виде стрелок вверх и вниз, позволяя пользователям голосовать за комментарий или против него.

Однако можно изменить стиль панели голосования. Текущие варианты — стандартные кнопки Вверх/Вниз или механизм голосования в стиле сердца.

Мы используем флаг **voteStyle** следующим образом:

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Enable Heart Button'; code-example-end]

Мы настоятельно рекомендуем делать это без кода, так как это также включает валидацию на стороне сервера. На странице настройки виджета см. раздел "Стиль голосования".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; title='Change Voting Style' app-screenshot-end]

Голосование также можно отключить, см. `Disable Voting` выше в опциях стиля.

---