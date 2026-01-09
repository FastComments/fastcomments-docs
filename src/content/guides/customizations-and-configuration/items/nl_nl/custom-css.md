[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments is ontworpen om aangepast te worden. De commentaar-widget zelf draait binnen een iframe om veiligheidsredenen, dus om aangepaste styling
toe te passen moet je een van twee benaderingen volgen.

De eerste, de gemakkelijkste benadering, en door ons geprefereerd, is het gebruik van de [widget-aanpassingspagina](https://fastcomments.com/auth/my-account/customize-widget).

In de widget-aanpassingspagina, zie de sectie "Geavanceerde opties weergeven", waaronder een gebied is met het label "Aangepaste CSS":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.custom-css'; title='Custom CSS Input Area' app-screenshot-end]

Deze aanpak heeft enkele voordelen:
1. De ingevoerde CSS wordt geminificeerd voordat deze naar de gebruiker wordt gestuurd, en de opmaak blijft consistent in de bewerkings-UI.
2. Je krijgt alle voordelen van de widget-aanpassings-UI, bijvoorbeeld het eenvoudig anders aanpassen van de commentaar-widget voor verschillende sites.
3. Wanneer we wijzigingen aan de commentaar-widget doorvoeren, wordt je aangepaste styling getest als onderdeel van ons releaseproces.

De tweede benadering is het opgeven van de **customCSS** parameter in de widgetconfiguratie, als volgt:

[code-example-start config = {customCSS: "button { background: red; }" }; linesToHighlight = [6]; title = 'Passing Custom CSS'; code-example-end]

Dit heeft echter *beperkingen*:
1. Er is een limiet aan hoeveel custom CSS er kan worden doorgegeven voordat onze servers het verzoek afwijzen, vanwege de grootte van de headers.
2. Je moet de custom CSS beheren in je eigen infrastructuur en build-systeem. Dit kan net zo goed een voordeel als een nadeel zijn.
3. Er is extra overhead doordat de custom CSS in dit geval **twee keer** over het netwerk wordt verzonden: eerst naar onze servers en vervolgens terug in de iframe-inhoud. Voor de meeste payloadgroottes is dit echter niet merkbaar.
4. Een veelgebruikte optimalisatie is het minificeren van de CSS om de netwerkgrootte te verkleinen; bij deze aanpak moet je dat echter zelf afhandelen.
5. Je custom CSS wordt niet getest wanneer wij wijzigingen doorvoeren.

### Externe CSS-bestanden

Je kunt de widget instrueren om een extern bestand op te halen met `@import`!

Het is aan te raden om de `@import` in een aanpassingsregel te plaatsen. Op deze manier, als we ooit een wijziging aan de commentaar-widget moeten doorvoeren, kunnen we onze automatiseringstools gebruiken om je setup te verifiëren. Dus bijvoorbeeld zou je een aanpassingsregel aanmaken in de Widget-aanpassings-UI, op `Advanced` klikken, en bij `Custom CSS` invoeren:

    @import url(https://example.com/styles.css);

#### In Code - Niet aanbevolen

Je kunt ook een extern CSS-bestand laden via de `customCSS`-eigenschap:

[code-example-start config = {customCSS: "@import url(https://example.com/styles.css);" }; linesToHighlight = [6]; title = 'External CSS File'; code-example-end]

Houd er echter rekening mee dat je CSS door ons niet getest kan worden als je dit doet. 

### Styling van gebruikersprofiel-modal

Gebruikersprofiel-modals kunnen ook worden gestyled met custom CSS. Om er echter voor te zorgen dat de aangepaste styling op gebruikersprofielen wordt toegepast, moeten alle CSS-selectors worden voorafgegaan door `.user-profile`. Zonder dit voorvoegsel wordt de aangepaste styling voor gebruikersprofiel-modals genegeerd.

Bijvoorbeeld:

[code-example-start config = {customCSS: ".user-profile .profile-name { color: blue; }" }; title = 'User Profile CSS'; code-example-end]

### Achterwaartse compatibiliteit

Bij FastComments weten we dat onze klanten de commentaar-widget aanpassen. Dat is met opzet zo - het laatste wat we willen is dat ons product ontwerpinconsistenties in jouw product veroorzaakt.

Omdat dit een belangrijk onderdeel van ons product is, hebben we een build-pijplijn waarmee we wijzigingen aan de commentaar-widget per klant bij elke release kunnen beoordelen.

Als we kleine problemen vinden, zullen we je account bijwerken om ervoor te zorgen dat onze release soepel verloopt. Als we grote brekende wijzigingen zien, kunnen we de release stilleggen.

---