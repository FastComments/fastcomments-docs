[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

Для аутентификации FastComments требует включённых сторонних cookie в вашем браузере. Без них пользователям всегда придётся указывать свой email, чтобы комментировать (если только поле ввода email не скрыто), а их комментарии по умолчанию будут помечаться как непроверенные.

Чтобы обойти это, можно включить обход сторонних cookie. 

При включении этой настройки появится небольшое всплывающее окно с сообщением о том, что пользователь проходит авторизацию. Это всплывающее окно появляется каждый раз, когда пользователь взаимодействует с виджетом комментариев; например, когда он оставляет комментарий.

Мы можем сделать это в коде, установив флаг **enableThirdPartyCookieBypass** в true:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Enabling Third-Party Cookie Bypass'; code-example-end]

Также это можно настроить через интерфейс настройки виджета, в пункте `Enable Third-Party Cookie Popup`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; title='Enabling Third-Party Cookie Bypass' app-screenshot-end]

---