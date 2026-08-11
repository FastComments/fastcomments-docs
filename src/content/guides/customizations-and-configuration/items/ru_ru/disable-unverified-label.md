---
[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

По умолчанию FastComments будет показывать метку «Неподтверждённый комментарий» для комментариев, оставленных пользователем с неподтверждённой сессией браузера. Подробнее о неподтверждённом комментировании читайте [здесь](https://docs.fastcomments.com/guide-comment-vote-verification.html).

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = 'Отключить метку неподтверждённого комментария'; code-example-end]

Кроме того, эту функцию можно использовать без написания кода в пользовательском интерфейсе настройки:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; alt='Страница настройки виджета с отмеченным флажком Отключить метку неподтверждённого комментария'; title='Отключить метку неподтверждённого комментария' app-screenshot-end]

---