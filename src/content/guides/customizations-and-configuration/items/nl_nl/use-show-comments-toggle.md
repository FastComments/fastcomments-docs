[related-parameter-start name = 'useShowCommentsToggle'; type = 'boolean'; related-parameter-end]

Standaard rendert FastComments het invoerveld voor opmerkingen en de opmerkingen-thread tegelijkertijd. Om wat verticale ruimte te besparen,
verbergt het ook andere vereiste velden totdat er interactie met de widget is.

De reacties-widget kan echter achter een knop worden verborgen, bijvoorbeeld:

[app-screenshot-start width=700; url=`https://fastcomments.com/embed?config=%7B%22tenantId%22%3A%22L177BUDVvSe%22%2C%22useShowCommentsToggle%22%3A%22true%22%2C%22urlId%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22url%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22pageTitle%22%3A%22%22%2C%22instanceId%22%3A%220.1281898364813452.1655790389169%22%7D&wId=comment-ui-v2`; selector = '.fast-comments'; delay=2000; title='Click to Show Comments' app-screenshot-end]

De knop gebruikt verschillende vertaalde teksten, afhankelijk van of de opmerkingen momenteel getoond worden of niet. Als de opmerkingen verborgen zijn, gebruikt het `translations.SHOW_COMMENTS_BUTTON_TEXT`. Als de
opmerkingen getoond worden, gebruikt het `translations.HIDE_COMMENTS_BUTTON_TEXT`. De vertalingen kunnen de tekst `[count]` bevatten, die
vervangen zal worden door de gelokaliseerde telling.

[code-example-start config = {useShowCommentsToggle: true}; linesToHighlight = [6]; title = 'Click to Show or Hide Comments'; code-example-end]

Dit is ontworpen ter vervanging van de `hideCommentsUnderCountTextFormat`-configuratie.

De telling wordt live bijgewerkt met de opmerkingen-thread. De knop wordt niet weergegeven als er geen opmerkingen zijn.

Dit kan zonder code worden ingeschakeld door een aanpassingsregel te maken en "Klik om opmerkingen te tonen" in te schakelen:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments']; selector = '.click-to-show-comments'; title='Enable Click to Show Comments' app-screenshot-end]