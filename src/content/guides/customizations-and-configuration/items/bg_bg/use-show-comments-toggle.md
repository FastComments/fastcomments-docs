[related-parameter-start name = 'useShowCommentsToggle'; type = 'boolean'; related-parameter-end]

По подразбиране FastComments ще показва полето за въвеждане на коментари и нишката с коментари едновременно. За да спести малко вертикално пространство,
той също така ще скрива всички други задължителни полета, докато не се взаимодейства с уиджета.

Въпреки това, коментарният уиджет може да бъде скрит зад бутон, например:

[app-screenshot-start width=700; url=`https://fastcomments.com/embed?config=%7B%22tenantId%22%3A%22L177BUDVvSe%22%2C%22useShowCommentsToggle%22%3A%22true%22%2C%22urlId%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22url%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22pageTitle%22%3A%22%22%2C%22instanceId%22%3A%220.1281898364813452.1655790389169%22%7D&wId=comment-ui-v2`; selector = '.fast-comments'; delay=2000; title='Click to Show Comments' app-screenshot-end]

Бутонът използва различен преведен текст в зависимост от това дали коментарите в момента са показани или не. Ако коментарите са скрити, се използва `translations.SHOW_COMMENTS_BUTTON_TEXT`. Ако коментарите са показани, се използва `translations.HIDE_COMMENTS_BUTTON_TEXT`. Преводите могат да съдържат текста `[count]`, който ще бъде заменен с локализирания брой.

[code-example-start config = {useShowCommentsToggle: true}; linesToHighlight = [6]; title = 'Click to Show or Hide Comments'; code-example-end]

Това е предназначено да замени конфигурацията `hideCommentsUnderCountTextFormat`.

Броят се обновява в реално време заедно с нишката с коментари. Бутонът не се показва, ако няма коментари.

Това може да бъде включено без код чрез създаване на правило за персонализиране и активиране на "Click to Show Comments":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments']; selector = '.click-to-show-comments'; title='Enable Click to Show Comments' app-screenshot-end]