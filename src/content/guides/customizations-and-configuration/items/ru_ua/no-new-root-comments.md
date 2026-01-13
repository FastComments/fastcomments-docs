[related-parameter-start name = 'noNewRootComments'; type = 'boolean'; related-parameter-end]

Установка `noNewRootComments` в `true` заставит виджет скрыть область корневого ответа, но при этом позволит пользователям отвечать
на дочерние комментарии. Вы, например, можете задать это условно при загрузке страницы, чтобы разрешить только некоторым пользователям оставлять комментарии верхнего уровня.

[code-example-start config = {noNewRootComments: true}; linesToHighlight = [6]; title = 'Prevent New Root Comments'; code-example-end]

---