По умолчанию FastComments позволяет пользователям удалять свои комментарии.

Однако это можно запретить.

На странице настройки виджета найдите опцию "Отключить удаление".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-delete'; selector = '.disable-commenter-comment-delete'; title='Disable Comment Deleting' app-screenshot-end]

- Это влияет только на обычных комментаторов и не затрагивает модераторов или администраторов, которые по-прежнему смогут удалять.
- Это также повлияет на интеграции через API в случаях, когда передаётся `contextUserId`.