[related-parameter-start name = 'useShowCommentsToggle'; type = 'boolean'; related-parameter-end]

Standaard rendert FastComments zowel het reactie‑invoerveld als de reactiedraad tegelijk. Om wat verticale ruimte te besparen,
verbergt het ook alle andere verplichte velden totdat er met de widget wordt gecommuniceerd.

De reactiewidget kan echter verborgen worden achter een knop, bijvoorbeeld:

[app-screenshot-start width=700; url=`https://fastcomments.com/embed?config=%7B%22tenantId%22%3A%22L177BUDVvSe%22%2C%22useShowCommentsToggle%22%3A%22true%22%2C%22urlId%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22url%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22pageTitle%22%3A%22%22%2C%22instanceId%22%3A%220.1281898364813452.1655790389169%22%7D&wId=comment-ui-v2`; selector = '.fast-comments'; delay=2000; alt='Reactiewidget samengevouwen achter een knop die het aantal reacties toont totdat een lezer erop klikt'; title='Klik om reacties te tonen' app-screenshot-end]

De knop gebruikt verschillende vertaalde teksten afhankelijk van of de reacties momenteel worden getoond of niet. Als de reacties verborgen zijn, wordt `translations.SHOW_COMMENTS_BUTTON_TEXT` gebruikt. Als de
reacties worden getoond, wordt `translations.HIDE_COMMENTS_BUTTON_TEXT` gebruikt. De vertalingen kunnen de tekst `[count]` bevatten die
wordt vervangen door het gelokaliseerde aantal.

[code-example-start config = {useShowCommentsToggle: true}; linesToHighlight = [6]; title = 'Klik om reacties te tonen of te verbergen'; code-example-end]

Dit is bedoeld om de `hideCommentsUnderCountTextFormat` configuratie te vervangen.

Het aantal wordt live bijgewerkt met de reactiedraad. De knop wordt niet getoond als er geen reacties zijn.

Dit kan worden ingeschakeld zonder code door een aanpassingsregel te maken en "Klik om reacties te tonen" in te schakelen:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments']; selector = '.click-to-show-comments'; alt='Selectievakje \'Klik om reacties te tonen\' aangevinkt in een aanpassingsregel op de widget‑aanpassingspagina'; title='Schakel Klik om reacties te tonen in' app-screenshot-end]