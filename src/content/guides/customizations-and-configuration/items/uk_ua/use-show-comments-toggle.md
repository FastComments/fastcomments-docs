[related-parameter-start name = 'useShowCommentsToggle'; type = 'boolean'; related-parameter-end]

За замовчуванням FastComments відображає поле введення коментаря та потік коментарів одночасно. Щоб заощадити вертикальний простір,
воно також приховує будь-які інші обов'язкові поля, поки з віджетом не взаємодіють.

Однак віджет коментарів можна приховати за кнопкою, наприклад:

[app-screenshot-start width=700; url=`https://fastcomments.com/embed?config=%7B%22tenantId%22%3A%22L177BUDVvSe%22%2C%22useShowCommentsToggle%22%3A%22true%22%2C%22urlId%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22url%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22pageTitle%22%3A%22%22%2C%22instanceId%22%3A%220.1281898364813452.1655790389169%22%7D&wId=comment-ui-v2`; selector = '.fast-comments'; delay=2000; title='Click to Show Comments' app-screenshot-end]

Кнопка використовує різний перекладений текст залежно від того, чи показані коментарі в даний момент. Якщо коментарі приховані, вона використовує `translations.SHOW_COMMENTS_BUTTON_TEXT`. Якщо
коментарі показані, вона використовує `translations.HIDE_COMMENTS_BUTTON_TEXT`. Переклади можуть містити текст `[count]`, який буде
замінено на локалізовану кількість.

[code-example-start config = {useShowCommentsToggle: true}; linesToHighlight = [6]; title = 'Click to Show or Hide Comments'; code-example-end]

Це призначено для заміни конфігурації `hideCommentsUnderCountTextFormat`.

Кількість оновлюється в реальному часі разом із потоком коментарів. Кнопка не відображається, якщо коментарів немає.

Цю опцію можна ввімкнути без коду, створивши правило налаштування та увімкнувши «Натисніть, щоб показати коментарі»:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments']; selector = '.click-to-show-comments'; title='Enable Click to Show Comments' app-screenshot-end]