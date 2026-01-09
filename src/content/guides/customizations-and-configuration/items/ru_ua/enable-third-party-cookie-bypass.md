[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

Для аутентификации FastComments требует, чтобы в вашем браузере были включены сторонние cookies. Без них пользователям всегда придётся
оставлять свой адрес электронной почты, чтобы комментировать (если поле ввода email не скрыто), а их комментарии по умолчанию будут отображаться как неподтверждённые.

Чтобы обойти это, вы можете включить обход блокировки сторонних cookies. 

Когда эта опция включена, будет появляться небольшое всплывающее окно с сообщением о входе пользователя в систему. Это всплывающее окно
появляется каждый раз, когда пользователь взаимодействует с виджетом комментариев; например, когда он оставляет комментарий.

Это можно сделать в коде, установив флаг **enableThirdPartyCookieBypass** в значение true:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Enabling Third-Party Cookie Bypass'; code-example-end]

Также это можно настроить через интерфейс настройки виджета (Widget Customization UI), в разделе `Enable Third-Party Cookie Popup`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; title='Enabling Third-Party Cookie Bypass' app-screenshot-end]