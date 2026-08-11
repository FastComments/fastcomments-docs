[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

Som standard vil FastComments kun vise brugerbadge på deres kommentarer inden for kommentartråden.

Men vi kan vise brugerbadge ved siden af deres navn over kommentarfeltet ved at aktivere denne funktion på widget-tilpasningssiden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; alt='Vis badge i topbjælke afkrydsningsfelt på widget-tilpasningssiden, placerer badge ved siden af navnet over kommentarfeltet'; title='Vis badge i topbjælke-indstilling' app-screenshot-end]

Dette vil vise brugerens badge ved siden af deres navn i topbjælkeområdet, så deres præstationer og status fremstår mere tydelige, når de skriver en kommentar.

Bemærk, at denne funktion skal være aktiveret i widget-tilpasnings‑UI’en for at fungere. Du kan valgfrit sætte **showBadgesInTopBar**‑flaget til false i din kodekonfiguration for selektivt at deaktivere den, selvom den er slået til på serverniveau:

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Deaktiver visning af badge i topbjælke'; code-example-end]