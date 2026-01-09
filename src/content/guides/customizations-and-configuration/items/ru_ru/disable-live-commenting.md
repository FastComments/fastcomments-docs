[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

По умолчанию в FastComments включено живое комментирование.

Это означает, что каждый, кто просматривает ветку комментариев, увидит одно и то же содержимое.

Например, если добавлен комментарий, этот комментарий должен отображаться. Если комментарий отредактирован или удалён,
то эти комментарии будут отредактированы или удалены для всех просматривающих ветку. То же самое относится к голосам и ко всем действиям модерации.

Однако это можно отключить:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disable Live Commenting'; code-example-end]

Это также можно сделать без кода. На странице настройки виджета см. раздел "Отключить живое комментирование".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; title='Disable Live Commenting' app-screenshot-end]

---