---
[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

По подразбиране FastComments ще показва етикет "Непотвърден коментар" за коментари, които са оставени за потребител, който
има непотвърдена сесия в браузъра. Прочетете повече за непотвърденото коментиране [тук](https://docs.fastcomments.com/guide-comment-vote-verification.html).

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = 'Disable The Unverified Label'; code-example-end]

Освен това тази функция може да се използва, без да се пише код, в интерфейса за персонализиране:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; title='Disable The Unverified Label' app-screenshot-end]

---