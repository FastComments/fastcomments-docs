[related-parameter-start name = 'useShowCommentsToggle'; type = 'boolean'; related-parameter-end]

Подразумевано, FastComments ће приказати поље за унос коментара и нит коментара истовремено. Да би уштедио неко вертикално простора,
такође ће сакрити свака друга потребна поља док се виџет не интерагује.

Међутим, виџет за коментаре може бити скривен иза дугмета, на пример:

[app-screenshot-start width=700; url=`https://fastcomments.com/embed?config=%7B%22tenantId%22%3A%22L177BUDVvSe%22%2C%22useShowCommentsToggle%22%3A%22true%22%2C%22urlId%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22url%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22pageTitle%22%3A%22%22%2C%22instanceId%22%3A%220.1281898364813452.1655790389169%22%7D&wId=comment-ui-v2`; selector = '.fast-comments'; delay=2000; alt='Виџет за коментаре склопљен иза дугмета које приказује број коментара док читалац не кликне на њега'; title='Кликните за приказ коментара' app-screenshot-end]

Дугме користи различит преведени текст у зависности од тога да ли су коментари тренутно приказани или не. Ако су коментари скривени, користи `translations.SHOW_COMMENTS_BUTTON_TEXT`. Ако
коментари су приказани, користи `translations.HIDE_COMMENTS_BUTTON_TEXT`. Преводи могу садржати текст `[count]` који ће
бити замењен локализованим бројем.

[code-example-start config = {useShowCommentsToggle: true}; linesToHighlight = [6]; title = 'Кликните за приказ или скривање коментара'; code-example-end]

Ово је осмишљено да замени конфигурацију `hideCommentsUnderCountTextFormat`.

Број се ажурира уживо са нитима коментара. Дугме се не приказује ако нема коментара.

Ово се може омогућити без кода креирањем правила прилагођавања и укључивањем „Кликните за приказ коментара“:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments']; selector = '.click-to-show-comments'; alt='Поље за потврду „Кликните за приказ коментара“ означено у правилу прилагођавања на страници за прилагођавање виџета'; title='Омогући Кликните за приказ коментара' app-screenshot-end]

---