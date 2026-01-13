[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

По умолчанию ответы на комментарии верхнего уровня отображаются.

Это можно настроить так, чтобы пользователю приходилось нажимать «Показать ответы» в комментариях верхнего уровня, чтобы увидеть дочерние.

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Collapse Replies to Top Level Comments'; code-example-end]

Это можно настроить без кода на странице настройки виджета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; title='Collapse Replies' app-screenshot-end]

Эта настройка не влияет на количество загружаемых первоначально комментариев верхнего уровня. Если у вас есть один комментарий верхнего уровня и 29 дочерних, при включённой этой настройке вы:

- Увидите комментарий верхнего уровня.
- Увидите «Показать ответы (29)» под этим комментарием.

Если вы хотите показать все комментарии верхнего уровня в сочетании с этой опцией, установите [starting page to -1](#starting-page).