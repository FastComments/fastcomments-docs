[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

За замовчуванням FastComments відображатиме мітку "Неперевірений коментар" для коментарів, які були залишені для користувача, у якого
неперевірена сесія браузера. Докладніше про неперевірені коментарі читайте [тут](https://docs.fastcomments.com/guide-comment-vote-verification.html).

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = 'Disable The Unverified Label'; code-example-end]

Крім того, цю функцію можна використовувати без написання коду в інтерфейсі налаштування:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; title='Disable The Unverified Label' app-screenshot-end]

---