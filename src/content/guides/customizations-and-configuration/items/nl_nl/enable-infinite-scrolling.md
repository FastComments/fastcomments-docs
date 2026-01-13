[related-parameter-start name = 'enableInfiniteScrolling'; type = 'boolean'; related-parameter-end]

Standaard zal de FastComments-widget zichzelf verticaal aanpassen om alle zichtbare reacties te tonen. Paginering wordt bereikt via een "Bekijk volgende"
knop aan het einde van de huidige pagina, omdat we hebben vastgesteld dat deze interactie voor de meeste gebruikers het prettigst aanvoelt.

Er zijn echter gevallen waarin oneindig scrollen de voorkeur heeft. Bijvoorbeeld: we gebruiken deze functie in ons Stream Chat-product.

We kunnen de "Bekijk volgende"-knoppen verbergen en overschakelen naar oneindig scrollen door de **enableInfiniteScrolling**-vlag op true te zetten:

[code-example-start config = {enableInfiniteScrolling: true}; linesToHighlight = [6]; title = 'Enabling Infinite Scrolling'; code-example-end]

Dit vereist ook het toevoegen van aangepaste CSS. Voeg bijvoorbeeld aangepaste CSS toe voor de `.comments`-selector om scrollen mogelijk te maken:

[inline-code-attrs-start title = 'Oneindig scrollen inschakelen'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
.comments {
    max-height: 500px;
    overflow-y: auto;
}
[inline-code-end]

Een volledig werkend voorbeeld zou zijn:

[code-example-start config = {enableInfiniteScrolling: true, customCSS: '.comments { max-height: 500px;  overflow-y: auto; }'}; linesToHighlight = [6, 7]; title = 'Enabling Infinite Scrolling'; code-example-end]

In het bovenstaande voorbeeld gebruiken we de `customCSS`-eigenschap, maar wordt aanbevolen om in plaats daarvan de Widget Configuration UI te gebruiken vanwege prestatieoverwegingen. [Zie de documentatie voor Custom CSS.](/guide-customizations-and-configuration.html#custom-css)

---