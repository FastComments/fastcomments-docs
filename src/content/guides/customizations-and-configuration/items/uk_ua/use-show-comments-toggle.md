[related-parameter-start name = 'useShowCommentsToggle'; type = 'boolean'; related-parameter-end]

За замовчуванням FastComments відображає поле вводу коментаря та гілку коментарів одночасно. Щоб заощадити вертикальний простір, він також приховує інші обов’язкові поля, доки користувач не взаємодіє з віджетом.

Однак віджет коментарів можна сховати за кнопкою, наприклад:

[app-screenshot-start width=700; url=`https://fastcomments.com/embed?config=%7B%22tenantId%22%3A%22L177BUDVvSe%22%2C%22useShowCommentsToggle%22%3A%22true%22%2C%22urlId%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22url%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22pageTitle%22%3A%22%22%2C%22instanceId%22%3A%220.1281898364813452.1655790389169%22%7D&wId=comment-ui-v2`; selector = '.fast-comments'; delay=2000; alt='Віджет коментарів згорнутий за кнопкою, яка показує кількість коментарів, доки читач не натисне її'; title='Click to Show Comments' app-screenshot-end]

Кнопка використовує різний перекладений текст залежно від того, чи показані коментарі в даний момент. Якщо коментарі приховані, використовується `translations.SHOW_COMMENTS_BUTTON_TEXT`. Якщо
коментарі показані, використовується `translations.HIDE_COMMENTS_BUTTON_TEXT`. Переклади можуть містити текст `[count]`, який буде
замінений локалізованою кількістю.

[code-example-start config = {useShowCommentsToggle: true}; linesToHighlight = [6]; title = 'Click to Show or Hide Comments'; code-example-end]

Це призначено для заміни конфігурації `hideCommentsUnderCountTextFormat`.

Кількість оновлюється в режимі реального часу разом з гілкою коментарів. Кнопка не відображається, якщо немає коментарів.

Це можна ввімкнути без коду, створивши правило налаштування та активувавши "Click to Show Comments":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments']; selector = '.click-to-show-comments'; alt='Прапорець «Показати коментарі» позначений у правилі налаштування на сторінці налаштування віджету'; title='Enable Click to Show Comments' app-screenshot-end]