[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

По умолчанию в FastComments включены живые комментарии.

Это означает, что каждый пользователь, просматривающий ветку комментариев, видит одинаковое содержимое.

Например, если комментарий добавлен, этот комментарий появится. Если комментарий отредактирован или удалён,
то эти изменения будут видны всем пользователям ветки. То же самое относится к голосам и всем действиям модерации.

Однако это можно отключить:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disable Live Commenting'; code-example-end]

Это также можно сделать без кода. На странице настройки виджета смотрите раздел «Disable Live Commenting».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; title='Disable Live Commenting' app-screenshot-end]

---