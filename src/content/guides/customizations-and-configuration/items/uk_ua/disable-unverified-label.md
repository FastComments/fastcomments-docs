[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

За замовчуванням FastComments показуватиме мітку «Неперевірений коментар» для коментарів, залишених користувачем, у якого неперевірена браузерна сесія. Дізнайтеся більше про неперевірені коментарі [тут](https://docs.fastcomments.com/guide-comment-vote-verification.html).

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = 'Вимкнути мітку неперевіреного коментаря'; code-example-end]

Крім того, цю функцію можна використовувати без написання коду в інтерфейсі налаштувань:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; alt='Сторінка налаштування віджету з позначеним прапорцем «Вимкнути мітку неперевіреного коментаря»'; title='Вимкнути мітку неперевіреного коментаря' app-screenshot-end]