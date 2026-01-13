[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

По умолчанию FastComments отображает колокольчик уведомлений в правом верхнем углу области комментариев.

Этот колокольчик становится красным и показывает количество уведомлений у пользователя. Примеры уведомлений:

- Пользователь ответил вам.
- Пользователь ответил в треде, в котором вы комментировали.
- Пользователь проголосовал за ваш комментарий.
- Пользователь ответил на странице, на которую вы подписаны.

Колокольчик уведомлений также позволяет подписываться на всю страницу.

Однако мы можем полностью отключить колокольчик уведомлений:

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Disable Notification Bell'; code-example-end]

Это также можно сделать без кода. На странице настройки виджета смотрите раздел «Отключить колокольчик уведомлений».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; title='Disable Notification Bell' app-screenshot-end]

---