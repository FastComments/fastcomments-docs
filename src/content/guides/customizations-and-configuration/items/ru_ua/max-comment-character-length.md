[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

Максимальное количество символов, которые разрешено вводить в поле для комментария, можно ограничить параметром **maxCommentCharacterLength**.

По умолчанию — 2000.

Такие элементы, как URL изображений, не включаются в определение длины.

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = 'Limit Comment Length'; code-example-end]

Это можно настроить без кода на странице настройки виджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; title='Limit Comment Length' app-screenshot-end]