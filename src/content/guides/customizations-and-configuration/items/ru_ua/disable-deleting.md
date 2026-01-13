---
По умолчанию FastComments позволяет пользователям удалять свои комментарии.

Однако это можно запретить.

На странице настройки виджета выберите опцию "Отключить удаление".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-delete'; selector = '.disable-commenter-comment-delete'; title='Disable Comment Deleting' app-screenshot-end]

- Это влияет только на обычных комментаторов (Commenters), а не на модераторов или администраторов — они по-прежнему смогут удалять.
- Это также влияет на интеграции API в случаях, когда передаётся `contextUserId`. 

---