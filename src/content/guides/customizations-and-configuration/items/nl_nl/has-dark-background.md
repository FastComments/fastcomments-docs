[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

Standaard detecteert de FastComments comment widget automatisch de donkere modus op de meeste sites.

Wanneer de donkere modus wordt gedetecteerd, schakelt FastComments over van zwarte tekst op witte achtergronden naar witte tekst op een zwarte achtergrond. Afbeeldingen zullen ook veranderen.

Bij het laden van de pagina zal de widget proberen te bepalen hoe donker de achtergrond van de pagina is achter de comment widget. Dit betekent dat
de pagina een witte achtergrond kan hebben, maar als u de comment widget in een container met een zwarte achtergrond plaatst, zou de donkere modus
nog steeds automatisch ingeschakeld moeten worden om de reacties leesbaar te maken.

Echter, het detectiemechanisme, dat afhankelijk is van het bepalen van de "luminantie", schakelt mogelijk de donkere modus niet in wanneer u dat wilt. Om deze geforceerd in te schakelen, zet de
*hasDarkBackground* vlag op true als volgt:

[code-example-start config = {hasDarkBackground: true}; linesToHighlight = [6]; title = 'Force Dark Background Mode'; additionalDemoCode = '<style>body { background: black; }</style>'; code-example-end]