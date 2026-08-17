[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

Когда пользователь комментирует с FastComments в первый раз, мы попытаемся получить его аватар с <a href="https://gravatar.com/" target="_blank">http://gravatar.com/</a>.

Однако, если аватар не найден, или пользователь никогда не задавал его в своей учётной записи, мы отображаем статическое изображение аватара по умолчанию.

Чтобы указать собственное статическое изображение аватара, можно использовать настройку *defaultAvatarSrc*.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Override The Default Avatar'; code-example-end]

Это также можно сделать без кода. На странице настройки виджета смотрите раздел «Default Avatar».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; alt='Раздел «Default Avatar» на странице настройки виджета, где вы задаёте URL резервного изображения аватара'; title='Настройка аватара по умолчанию' app-screenshot-end]

Обратите внимание, что определение аватара для конкретного пользователя, например через SSO, рассматривается в отдельном разделе.