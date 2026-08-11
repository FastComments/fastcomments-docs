[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

Для аутентификации FastComments зависит от того, что в вашем браузере включены сторонние cookie. Без них пользователи всегда будут вынуждены оставлять свой email для комментирования (если поле ввода email не скрыто), и их комментарии всегда будут отображаться как непроверенные (по умолчанию).

Чтобы обойти это, вы можете включить обход сторонних cookie. 

Когда эта настройка включена, появляется небольшое всплывающее окно, показывающее сообщение о том, что пользователь входит в систему. Это всплывающее окно появляется каждый раз, когда пользователь взаимодействует с виджетом комментариев; например, когда он оставляет комментарий.

Мы можем сделать это в коде, установив флаг **enableThirdPartyCookieBypass** в true:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Включение обхода сторонних cookie'; code-example-end]

Мы также можем настроить это через пользовательский интерфейс настройки виджета, в разделе `Enable Third-Party Cookie Popup`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; alt='Страница настройки виджета с отмеченным флажком «Включить всплывающее окно сторонних cookie»'; title='Включение обхода сторонних cookie' app-screenshot-end]