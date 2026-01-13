[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

По умолчанию FastComments отображает значок уведомлений в правом верхнем углу области комментариев.

Этот значок станет красным и покажет количество уведомлений у пользователя. Примеры уведомлений:

- Пользователь ответил вам.
- Пользователь ответил в ветке, в которой вы комментировали.
- Пользователь проголосовал за ваш комментарий.
- Пользователь ответил на странице, на которую вы подписаны.

Значок уведомлений также предоставляет механизм подписки на всю страницу.

Однако можно полностью отключить значок уведомлений:

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Disable Notification Bell'; code-example-end]

Это также можно сделать без кода. На странице настройки виджета см. раздел "Disable Notification Bell".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; title='Disable Notification Bell' app-screenshot-end]