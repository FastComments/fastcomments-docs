[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Wanneer een reactiedraad wordt weergegeven, of wanneer een reactie wordt geplaatst, moet FastComments weten op welke pagina, artikel of product
die reacties betrekking hebben.

Om dit te doen gebruiken we iets dat we de "URL ID" noemen. Het is ofwel een identificator, zoals een string of een getal, of een URL.

Standaard, als je de urlId niet specificeert, wordt deze de pagina-URL. We zullen de huidige pagina-URL nemen en deze opschonen om
veelvoorkomende marketingparameters of trackingidentifiers te verwijderen.

In het geval van integraties van derden, zoals WordPress, zal onze plugin gewoonlijk de identifier gebruiken die de huidige bekeken informatie vertegenwoordigt als
de URL ID, bijvoorbeeld de artikel-/pagina-id.

[code-example-start config = {urlId: 'https://example.com/page'}; linesToHighlight = [6]; title = 'Defining a Custom URL ID'; code-example-end]

Eén ding waar we vaak naar verwijzen in dit document is de <a href="https://fastcomments.com/auth/my-account/customize-widget/new">UI voor widgetaanpassing</a>.

Met deze UI kun je veel wijzigingen aanbrengen in de reactie-widget zonder code te gebruiken.

Wanneer we een aanpassingsregel maken, willen we die vaak toepassen op alle pagina's van onze site. In sommige gevallen willen we echter het comment-widget
op een specifieke pagina aanpassen, bijvoorbeeld om aangepaste styling toe te passen, of misschien om reacties voor die specifieke pagina anoniem te maken. Je zou ook, bijvoorbeeld, live reacties
direct op sommige pagina's kunnen laten verschijnen, terwijl je ze op andere pagina's onder notificatieknoppen verbergt.

Dit is allemaal mogelijk via het URL ID-invoerveld op deze pagina, dat er als volgt uitziet:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.url-id'; title='URL ID Input in The Widget Customization Page' app-screenshot-end]

De waarde in dit veld moet overeenkomen met de *urlId* parameter die aan het comment-widget wordt doorgegeven. Als je wilt dat je aanpassingsregel *urlId* agnostisch is, laat je dit veld leeg of voer je * in.

As of 2023 the `URL ID` field in widget customization now also takes patterns! For example you may
have `*/blog/*` to add styling specific to your blog and `*/store/*` to have styling specific to your store,
all while using the same domain.

### Valkuilen

1. Als je pagina hashparameters heeft (zoals example.com#page-1) - wordt dit standaard onderdeel van de URL ID.
2. Tijdens migraties, bijvoorbeeld van WordPress naar Gatsby, moet je mogelijk de URL ID-waarden van reacties migreren na de initiële migratie. Neem daarvoor contact met ons op.