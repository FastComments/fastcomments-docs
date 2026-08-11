[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

Standaard zal FastComments de commentaarwidget weergeven in de locale die wordt bepaald door het systeem en de browser van de gebruiker.

Wanneer een gebruiker een reactie plaatst of inlogt, werken we hun laatst gebruikte locale bij en gebruiken we deze ook voor het verzenden van e‑mails.

Dit beïnvloedt hoe de commentaarwidget voor de gebruiker wordt vertaald. Een locale bestaat uit de taal en regio van de gebruiker, dus het configureren van de locale zal meestal de taal wijzigen die wordt gebruikt om tekst aan de gebruiker te tonen.

#### Via de UI

Dit kan worden gedefinieerd via de UI voor widgetaanpassing. Zie de optie "Locale / Language" :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; alt='Locale / Language dropdown op de widget-aanpassingspagina die wordt gebruikt om de gedetecteerde locale van de bezoeker te overschrijven'; title='De locale / taal wijzigen' app-screenshot-end]

#### Via code

Dit kan worden overschreven met een gewenste locale.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Handmatig definiëren van de locale van de gebruiker'; code-example-end]

### Ondersteunde talen en locale-codes

[U kunt de volledige lijst met ondersteunde talen en de bijbehorende locale-codes hier vinden.](/guide-supported-languages.html#supported-languages)

### SSO-opmerking

Als u SSO gebruikt, wilt u mogelijk de locale van de gebruiker doorgeven in het gebruikersobject, zodat e‑mails en andere zaken correct voor hen worden gelokaliseerd.

---