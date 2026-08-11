[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

Standaard sorteert FastComments opmerkingen op de sorteer richting "Meest Relevant".

Meest Relevant sortering houdt rekening met de tijd waarop de opmerking is geplaatst en het aantal stemmen bij het sorteren.

De gebruiker kan vervolgens de sorteer richting wijzigen naar Oudste of Nieuwste eerst in de UI van de opmerking widget.

We kunnen de standaard echter wijzigen naar een van de drie. Bijvoorbeeld als je de oudste opmerkingen eerst wilt weergeven:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'De standaard sortering wijzigen naar oudste eerst'; code-example-end]

We stellen de waarde van **defaultSortDirection** in op "OF" om de richting in te stellen op "OF".

Voor de nieuwste-eerst sorteer richting zouden we het volgende doen:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'De standaard sortering wijzigen naar nieuwste eerst'; code-example-end]

De geldige waarden voor **defaultSortDirection** zijn:

- MR: "Meest Recent"
- NF: "Nieuwste eerst"
- OF: "Oudste eerst"

Dit kan ook zonder code worden gedaan. Op de widget-aanpassingspagina, zie de sectie "Standaard sorteer richting".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; alt='Standaard sorteer richting selector die Meest Relevant, Nieuwste eerst en Oudste eerst aanbiedt'; title='De standaard sorteer richting wijzigen' app-screenshot-end]

Merk op dat de opmerkingen op elke pagina voor elke sorteer richting vooraf worden berekend, zodat alle sorteer richtingen dezelfde prestaties hebben.