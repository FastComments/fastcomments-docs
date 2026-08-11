[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments is ontworpen om aangepast te kunnen worden. De commentaarwidget zelf draait binnen een iframe om veiligheidsredenen, dus om aangepaste styling toe te passen moet je een van twee benaderingen volgen.

De eerste, de gemakkelijkste benadering, en door ons geprefereerd, is om de [widget-aanpassingspagina](https://fastcomments.com/auth/my-account/customize-widget) te gebruiken.

Op de widget-aanpassingspagina, zie de sectie "Geavanceerde opties weergeven", waaronder een gebied staat gelabeld "Aangepaste CSS":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.custom-css'; alt='Aangepaste CSS-editor onder Toon geavanceerde opties op de widget-aanpassingspagina'; title='Aangepast CSS invoerveld' app-screenshot-end]

Deze benadering heeft enkele voordelen:
1. De ingevoerde CSS wordt geminimaliseerd voordat deze naar de gebruiker wordt verzonden, en de opmaak blijft consistent in de bewerkings-UI.
2. Je krijgt alle voordelen van de widget-aanpassings-UI, bijvoorbeeld het eenvoudig aanpassen van de commentaarwidget verschillend voor verschillende sites.
3. Wanneer we wijzigingen aan de commentaarwidget aanbrengen, wordt jouw aangepaste styling getest als onderdeel van ons releaseproces.

De tweede benadering is om de **customCSS**-parameter op te geven in de widgetconfiguratie, als volgt:

[code-example-start config = {customCSS: "button { background: red; }" }; linesToHighlight = [6]; title = 'Doorgeven van aangepaste CSS'; code-example-end]

Dit heeft echter *beperkingen*:
1. Er is een limiet aan hoeveel aangepaste CSS kan worden doorgegeven voordat onze servers het verzoek afwijzen, vanwege de grootte van de headers.
2. Je moet de aangepaste CSS beheren in je infrastructuur en buildsysteem. Dit kan zelfs een voordeel zijn in plaats van een nadeel.
3. Er is een extra overhead van het verzenden van de aangepaste CSS over het netwerk **twee keer** in dit geval, omdat het eerst naar onze servers moet worden gestuurd en vervolgens terug in de iframe-inhoud. Voor de meeste payloadgroottes is dit echter niet merkbaar.
4. Een veelvoorkomende optimalisatie is het minificeren van de CSS om de grootte over het netwerk te verkleinen, maar met deze benadering moet je dat zelf afhandelen.
5. Jouw aangepaste CSS wordt niet getest wanneer we wijzigingen aanbrengen.

### Externe CSS-bestanden

Je kunt de widget laten een extern bestand ophalen met `@import`!

Het wordt aanbevolen om de `@import` in een aanpassingsregel te plaatsen. Op deze manier, als we ooit een wijziging aan de commentaarwidget moeten aanbrengen, kunnen we onze automatiseringstools gebruiken om je configuratie te verifiëren. Dus bijvoorbeeld, je zou een aanpassingsregel maken in de Widget Customization UI, klik op `Advanced`, en voer in `Custom CSS`:

    @import url(https://example.com/styles.css);

#### In Code - Niet Aanbevolen

Je kunt ook een extern CSS-bestand laden via de `customCSS`-eigenschap:

[code-example-start config = {customCSS: "@import url(https://example.com/styles.css);" }; linesToHighlight = [6]; title = 'Extern CSS-bestand'; code-example-end]

Houd er echter rekening mee dat je CSS niet door ons kan worden getest als je dit doet.

### Styling van gebruikersprofiel-modals

Gebruikersprofiel-modals kunnen ook worden gestyled met aangepaste CSS. Om er echter voor te zorgen dat aangepaste styling wordt toegepast op gebruikersprofielen, moeten alle CSS-selectors worden voorafgegaan door `.user-profile`. Zonder dit voorvoegsel wordt aangepaste styling genegeerd voor gebruikersprofiel-modals.

Bijvoorbeeld:

[code-example-start config = {customCSS: ".user-profile .profile-name { color: blue; }" }; title = 'Gebruikersprofiel CSS'; code-example-end]

### Achterwaartse compatibiliteit

Bij FastComments weten we dat onze klanten de commentaarwidget aanpassen. Dat is zo ontworpen - het laatste wat we willen is dat ons product ontwerp-inconsistenties in jouw product veroorzaakt.

Aangezien dit een belangrijk onderdeel van ons product is, hebben we een build-pijplijn die ons in staat stelt om wijzigingen aan de commentaarwidget per klant te beoordelen bij elke release.

Als we kleine problemen vinden, zullen we je account bijwerken om ervoor te zorgen dat onze release soepel verloopt. Als we grote brekende wijzigingen zien, stelt dit ons in staat de release te stoppen.

---