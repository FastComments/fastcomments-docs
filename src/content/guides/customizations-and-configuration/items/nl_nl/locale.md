[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

Standaard zal FastComments de commentaar-widget weergeven in de locale die wordt bepaald door het systeem en de browser van de gebruiker.

Wanneer een gebruiker een reactie plaatst of inlogt, werken we hun laatst gebruikte locale bij en gebruiken we deze ook voor het versturen van e-mails.

Dit beïnvloedt hoe de commentaar-widget voor de gebruiker wordt vertaald. Locale bestaat uit de taal en de regio van de gebruiker, dus het configureren van de locale zal meestal de taal wijzigen die wordt gebruikt om tekst aan de gebruiker te tonen.

#### Via The UI

Dit kan worden ingesteld met behulp van de widget-aanpassings-UI. Zie de optie "Locale / Language":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; title='Changing The Locale / Language' app-screenshot-end]

#### Via Code

Dit kan worden overschreven met een gewenste locale.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Manually Defining the User\'s Locale'; code-example-end]

### Supported Languages and Locale Codes

[Je kunt hier de volledige lijst met ondersteunde talen en de bijbehorende locale-codes vinden.](/guide-supported-languages.html#supported-languages)

### SSO Note

Als je SSO gebruikt, wil je mogelijk de locale van de gebruiker doorgeven in het user object, zodat e-mails en andere zaken correct voor hen worden gelokaliseerd.