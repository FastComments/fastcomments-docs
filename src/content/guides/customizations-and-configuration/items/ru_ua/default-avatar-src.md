[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

Когда пользователь впервые комментирует через FastComments, мы попытаемся получить его аватар с <a href="http://gravatar.com/" target="_blank">http://gravatar.com/</a>.

Однако, если аватар не найден или пользователь никогда не устанавливал его в своём аккаунте, мы отображаем статическое изображение аватара по умолчанию.

Чтобы указать собственное статическое изображение аватара, можно использовать настройку *defaultAvatarSrc*.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Override The Default Avatar'; code-example-end]

Это также можно сделать без кода. На странице настройки виджета смотрите раздел "Аватар по умолчанию".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; title='Customizing The Default Avatar' app-screenshot-end]

Обратите внимание, что определение аватара для конкретного пользователя, например при использовании SSO, рассматривается в отдельном разделе.

---