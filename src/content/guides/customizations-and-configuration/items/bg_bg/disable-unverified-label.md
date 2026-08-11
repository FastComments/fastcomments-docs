[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

По подразбиране FastComments ще показва етикет „Unverified Comment“ за коментари, оставени за потребител, който има непроверена браузър сесия. Прочетете повече за непровереното коментиране [тук](https://docs.fastcomments.com/guide-comment-vote-verification.html).

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = 'Деактивиране на етикета за непроверен коментар'; code-example-end]

Освен това, тази функция може да се използва, без писане на код, в потребителския интерфейс за персонализиране:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; alt='Страница за персонализиране на уиджет с отметка за деактивиране на етикета за непроверен коментар'; title='Деактивиране на етикета за непроверен коментар' app-screenshot-end]

---