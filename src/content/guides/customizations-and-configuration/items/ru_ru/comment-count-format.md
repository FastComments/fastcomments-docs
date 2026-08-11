[related-parameter-start name = 'commentCountFormat'; type = 'string'; related-parameter-end]

Отображаемое количество комментариев в верхней части виджета комментариев можно настроить.

Это можно заменить любой строкой, и значение **[count]** будет заменено на количество комментариев, локализованное для пользователя.

[code-example-start config = {commentCountFormat: "There are [count] comments."}; linesToHighlight = [6]; title = 'Настройка текста количества комментариев'; code-example-end]

Это можно настроить без кода на странице настройки виджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.comment-count'; alt='Поле текста количества комментариев на странице настройки виджета, где [count] заменяется текущим общим числом'; title='Настройка текста количества комментариев' app-screenshot-end]