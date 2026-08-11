[related-parameter-start name = 'disableProfiles'; type = 'boolean'; related-parameter-end]

По умолчанию FastComments будет показывать профиль пользователя, когда вы нажимаете на его аватар.

Однако мы можем отключить эту функцию:

[code-example-start config = {disableProfiles: true}; linesToHighlight = [6]; title = 'Отключить профили'; code-example-end]

Это также можно сделать без кода. На странице настройки виджета смотрите раздел «Отключить профили».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profiles']; selector = '.disable-profiles'; alt='Страница настройки виджета с отмеченным флажком Отключить профили, поэтому аватары больше не открывают профили'; title='Отключить профили' app-screenshot-end]