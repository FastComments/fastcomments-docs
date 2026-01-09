[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

Standaard sorteert FastComments reacties op basis van de "Most Relevant" sorteer­richting.

De "Most Relevant"-sortering houdt rekening met het tijdstip waarop de reactie is geplaatst en het aantal stemmen bij het sorteren.

De gebruiker kan vervolgens in de comment-widget UI de sorteer­richting wijzigen naar ofwel Oldest of Newest First.

We kunnen de standaardwaarde echter naar elk van de drie wijzigen. Bijvoorbeeld als je de oudste reacties eerst wilt tonen:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Oldest First'; code-example-end]

We stellen de waarde van **defaultSortDirection** in op "OF" om de richting op "OF" te zetten.

Voor de newest-first sorteer­richting zouden we het volgende doen:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Newest First'; code-example-end]

De geldige waarden voor **defaultSortDirection** zijn:

- MR: "Most Recent"
- NF: "Newest First"
- OF: "Oldest First"

Dit kan ook zonder code. Op de pagina voor widget‑aanpassing, zie de sectie "Standaard sorteerrichting".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; title='Changing The Default Sort Direction' app-screenshot-end]

Let op: de reacties op elke pagina voor elke sorteer­richting worden vooraf berekend, dus alle sorteer­richtingen hebben dezelfde prestaties.