[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

По умолчанию FastComments будет показывать метку "Непроверенный комментарий" для комментариев, которые были оставлены для пользователя, у которого
сеанс браузера не подтверждён. Подробнее о непроверенных комментариях читайте [здесь](https://docs.fastcomments.com/guide-comment-vote-verification.html).

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = 'Disable The Unverified Label'; code-example-end]

Кроме того, эту функцию можно использовать без написания кода в интерфейсе Customization UI:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; title='Disable The Unverified Label' app-screenshot-end]

---