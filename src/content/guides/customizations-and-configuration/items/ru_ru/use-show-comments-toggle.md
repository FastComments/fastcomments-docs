[related-parameter-start name = 'useShowCommentsToggle'; type = 'boolean'; related-parameter-end]

По умолчанию FastComments отображает поле ввода комментария и ветку комментариев одновременно. Чтобы сэкономить вертикальное пространство, он также скрывает все остальные обязательные поля, пока пользователь не взаимодействует с виджетом.

Однако виджет комментариев может быть скрыт за кнопкой, например:

[app-screenshot-start width=700; url=`https://fastcomments.com/embed?config=%7B%22tenantId%22%3A%22L177BUDVvSe%22%2C%22useShowCommentsToggle%22%3A%22true%22%2C%22urlId%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22url%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22pageTitle%22%3A%22%22%2C%22instanceId%22%3A%220.1281898364813452.1655790389169%22%7D&wId=comment-ui-v2`; selector = '.fast-comments'; delay=2000; alt='Виджет комментариев свернут за кнопкой, которая показывает количество комментариев, пока читатель не нажмёт её'; title='Нажмите, чтобы показать комментарии' app-screenshot-end]

Кнопка использует разный переводимый текст в зависимости от того, отображаются ли комментарии в данный момент. Если комментарии скрыты, используется `translations.SHOW_COMMENTS_BUTTON_TEXT`. Если комментарии отображаются, используется `translations.HIDE_COMMENTS_BUTTON_TEXT`. Переводы могут содержать текст `[count]`, который будет заменён локализованным счётчиком.

[code-example-start config = {useShowCommentsToggle: true}; linesToHighlight = [6]; title = 'Нажмите, чтобы показать или скрыть комментарии'; code-example-end]

Это предназначено для замены конфигурации `hideCommentsUnderCountTextFormat`.

Счётчик обновляется в реальном времени вместе с веткой комментариев. Кнопка не отображается, если комментариев нет.

Это можно включить без кода, создав правило настройки и включив «Показать комментарии по нажатию»:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments']; selector = '.click-to-show-comments'; alt='Флажок «Показать комментарии по нажатию» отмечен в правиле настройки на странице настройки виджета'; title='Включить показ комментариев по нажатию' app-screenshot-end]