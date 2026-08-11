[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Wanneer een commentaarthread wordt weergegeven of een commentaar wordt geplaatst, moet FastComments weten bij welke pagina, artikel of product die commentaren horen.

Hiervoor gebruiken we iets dat we de “URL ID” noemen. Het is ofwel een identificatie, zoals een string of een getal, of een URL.

Standaard, als je geen urlId opgeeft, wordt dit de pagina‑URL. We nemen de huidige pagina‑URL en maken deze schoon om gemeenschappelijke marketing‑parameters of tracking‑identifiers te verwijderen.

In het geval van integraties van derden, zoals WordPress, zal onze plug‑in meestal de identifier gebruiken die de huidige bekeken informatie vertegenwoordigt als de URL ID, bijvoorbeeld de artikel‑/pagina‑id.

[code-example-start config = {urlId: 'https://example.com/page'}; linesToHighlight = [6]; title = 'Definiëren van een aangepaste URL-ID'; code-example-end]

Een ding dat we vaak in dit document zullen verwijzen is de <a href="https://fastcomments.com/auth/my-account/customize-widget/new">Widget Customization UI</a>.

Deze UI kan worden gebruikt om veel wijzigingen aan de commentaarwidget aan te brengen zonder code te gebruiken.

Wanneer we een aanpassingsregel maken, willen we die vaak toepassen op alle pagina's van onze site. In sommige gevallen willen we echter de commentaarwidget aanpassen op een specifieke pagina, bijvoorbeeld om aangepaste styling toe te passen, of om commentaren voor die specifieke pagina anoniem te maken. Je zou bijvoorbeeld ook live‑commentaren direct op sommige pagina's kunnen laten verschijnen, terwijl je ze op andere onder notificatie‑knoppen verbergt.

Dit is allemaal mogelijk via het URL ID‑invoerveld op deze pagina, dat er als volgt uitziet:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.url-id'; alt='URL-ID-veld gebruikt om een aanpassingsregel te beperken tot één pagina, of tot een patroon zoals */blog/*'; title='URL-ID-invoer op de widget-aanpassingspagina' app-screenshot-end]

De waarde in dit veld moet overeenkomen met de *urlId*‑parameter die aan de commentaarwidget wordt doorgegeven. Als je wilt dat je aanpassingsregel *urlId*‑agnostisch is, laat dit veld dan leeg of voer * in.

Vanaf 2023 accepteert het `URL ID`‑veld in widget‑aanpassing nu ook patronen! Bijvoorbeeld, je kunt `*/blog/*` gebruiken om styling specifiek voor je blog toe te voegen en `*/store/*` om styling specifiek voor je winkel toe te passen, allemaal terwijl je hetzelfde domein gebruikt.

### Valkuilen

1. Als je pagina hash‑parameters heeft (zoals example.com#page-1) – dit wordt standaard onderdeel van de URL ID.
2. Tijdens migraties, bijvoorbeeld van WordPress naar Gatsby, moet je mogelijk de URL ID‑commentaarwaarden migreren na de initiële migratie. Neem hiervoor contact met ons op.

---