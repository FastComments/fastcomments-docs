[related-parameter-start name = 'commentCountFormat'; type = 'string'; related-parameter-end]

Количество комментариев, отображаемое в верхней части виджета комментариев, можно настроить.

Её можно заменить любой строкой, а значение **[count]** будет заменено на число комментариев, локализованное для пользователя.

[code-example-start config = {commentCountFormat: "There are [count] comments."}; linesToHighlight = [6]; title = 'Customizing The Comment Count Text'; code-example-end]

Это можно настроить без кода на странице настройки виджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.comment-count'; title='Customizing The Comment Count Text' app-screenshot-end]