[related-parameter-start name = 'disableProfileDirectMessages'; type = 'boolean'; related-parameter-end]

По умолчанию FastComments будет показывать вкладку «Прямые сообщения» в профилях пользователей, позволяя посетителям отправлять пользователю прямые сообщения.

Однако мы можем отключить эту вкладку:

[code-example-start config = {disableProfileDirectMessages: true}; linesToHighlight = [6]; title = 'Disable Profile Direct Messages'; code-example-end]

Это также можно сделать без кода. На странице настройки виджета смотрите раздел «Отключить прямые сообщения».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profile-direct-messages']; selector = '.disable-profile-direct-messages'; alt='Страница настройки виджета с отмеченным флажком «Отключить прямые сообщения», скрывающим вкладку сообщений профиля'; title='Отключить прямые сообщения профиля' app-screenshot-end]