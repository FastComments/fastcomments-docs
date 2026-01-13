По умолчанию FastComments позволяет пользователям редактировать свои комментарии.

Однако это можно запретить.

На странице настройки виджета найдите опцию «Отключить редактирование».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-edit'; selector = '.disable-commenter-comment-edit'; title='Disable Comment Editing' app-screenshot-end]

- Это влияет только на обычных комментаторов и не затрагивает модераторов или администраторов, которые по-прежнему смогут редактировать.
- Это также повлияет на интеграции через API, когда передаётся `contextUserId`.