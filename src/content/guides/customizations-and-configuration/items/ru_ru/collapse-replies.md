[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

По умолчанию ответы на комментарии верхнего уровня отображаются.

Это можно настроить так, чтобы пользователь должен был нажать "Показать ответы" в верхнеуровневых комментариях, чтобы увидеть дочерние.

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Collapse Replies to Top Level Comments'; code-example-end]

Это можно настроить без кода, на странице настройки виджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; title='Collapse Replies' app-screenshot-end]

Эта настройка не влияет на количество загружаемых изначально верхнеуровневых комментариев. Если у вас есть один верхнеуровневый комментарий и 29 дочерних, при включенной этой настройке вы увидите:

- Увидите верхнеуровневый комментарий.
- Увидите «Показать ответы (29)» под этим комментарием.

Если вы хотите показать все верхнеуровневые комментарии в сочетании с этой опцией, установите [начальную страницу в -1](#starting-page).