[related-parameter-start name = 'noNewRootComments'; type = 'boolean'; related-parameter-end]

Установка `noNewRootComments` в `true` приведёт к тому, что виджет скроет область ответа для корневых комментариев, но всё ещё позволит пользователям отвечать
на дочерние комментарии. Вы могли бы, например, установить это условно при загрузке страницы, чтобы разрешать оставлять корневые комментарии только некоторым пользователям.

[code-example-start config = {noNewRootComments: true}; linesToHighlight = [6]; title = 'Prevent New Root Comments'; code-example-end]