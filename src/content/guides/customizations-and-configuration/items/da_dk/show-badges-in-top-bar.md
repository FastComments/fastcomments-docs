[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

Som standard viser FastComments bruger-badges kun på deres kommentarer i kommentarsamtalen.

Vi kan dog vise bruger-badges ved siden af deres navn over kommentarfeltet ved at aktivere denne funktion på siden til tilpasning af widgeten:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; title='Show Badges in Top Bar Option' app-screenshot-end]

Dette viser brugerens badges ved siden af deres navn i topbjælken, så deres præstationer og status fremhæves, når de skriver en kommentar.

Bemærk, at denne funktion skal være aktiveret i widgetens tilpasnings‑UI for at fungere. Du kan valgfrit sætte flaget **showBadgesInTopBar** til false i din kodekonfiguration for selektivt at deaktivere det, selv når det er slået til på serverniveau:

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Disable Show Badges in Top Bar'; code-example-end]