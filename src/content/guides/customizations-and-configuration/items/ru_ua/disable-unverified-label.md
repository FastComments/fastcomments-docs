---
[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

По умолчанию FastComments будет показывать метку "Unverified Comment" для комментариев, оставленных для пользователя, у которого
есть непроверенная сессия браузера. Подробнее о непроверенных комментариях см. [здесь](https://docs.fastcomments.com/guide-comment-vote-verification.html).

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = 'Disable The Unverified Label'; code-example-end]

Кроме того, эту функцию можно использовать, не написав ни строчки кода, в интерфейсе Customization UI:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; title='Disable The Unverified Label' app-screenshot-end]

---