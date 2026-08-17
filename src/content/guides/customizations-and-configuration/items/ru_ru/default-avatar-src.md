[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

Когда пользователь впервые комментирует с FastComments, мы попытаемся получить его аватар с <a href="https://gravatar.com/" target="_blank">https://gravatar.com/</a>.

Однако, если мы не найдем аватар, или пользователь никогда не задает его в своей учетной записи, мы отображаем статическое изображение аватара по умолчанию.

Чтобы указать собственное статическое изображение аватара, можно использовать настройку *defaultAvatarSrc*.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Переопределить аватар по умолчанию'; code-example-end]

Это также можно сделать без кода. На странице настройки виджета смотрите раздел «Default Avatar».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; alt='Раздел «Default Avatar» на странице настройки виджета, где вы задаёте URL резервного изображения аватара'; title='Настройка аватара по умолчанию' app-screenshot-end]

Обратите внимание, что определение аватара для конкретного пользователя, например с помощью SSO, рассматривается в отдельном разделе.