[related-parameter-start name = 'disableProfileComments'; type = 'boolean'; related-parameter-end]

По умолчанию FastComments будет показывать вкладку «Комментарии к профилю» в профилях пользователей, позволяя посетителям оставлять комментарии в чьем‑то профиле.

Однако мы можем отключить эту вкладку:

[code-example-start config = {disableProfileComments: true}; linesToHighlight = [6]; title = 'Отключить комментарии к профилю'; code-example-end]

Это также можно сделать без кода. На странице настройки виджета смотрите раздел «Отключить комментарии к профилю».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profile-comments']; selector = '.disable-profile-comments'; alt='Страница настройки виджета с отмеченным флажком «Отключить комментарии к профилю», скрывающим вкладку комментариев к профилю'; title='Отключить комментарии к профилю' app-screenshot-end]