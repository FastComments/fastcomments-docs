---
[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Standaard toont FastComments geen lijst met gebruikers op de pagina.

Je kunt een lijst weergeven van mensen die momenteel de pagina bekijken, naast de commentaarwidget. De lijst wordt live bijgewerkt wanneer gebruikers binnenkomen en vertrekken, en toont hun naam, avatar en een online‑indicator.

Er zijn drie lay‑outopties:

- `1` - Top: een horizontale rij overlappende avatars weergegeven boven de reacties.
- `2` - Left: een zijbalk met namen en online stippen weergegeven links van de widget.
- `3` - Right: dezelfde zijbalk weergegeven rechts van de widget.

Stel de **usersListLocation**‑vlag in om de functie in te schakelen:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Toon gebruikerslijst aan de rechterkant'; code-example-end]

Standaard toont de lijst alleen gebruikers die momenteel online zijn. Om ook mensen op te nemen die in het verleden op de pagina hebben gereageerd (maar nu niet bekijken), stel **usersListIncludeOffline** in op true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Inclusief eerdere commentatoren'; code-example-end]

Eerdere commentatoren worden weergegeven zonder de groene online stip, zodat duidelijk is wie er op dit moment aanwezig is.

Gebruikers met privéprofielen worden getoond met een generieke avatar en een "Privéprofiel"-label, zodat het aantal nauwkeurig blijft zonder identiteiten te onthullen.

Dit kan ook worden geconfigureerd zonder code. Op de widget‑aanpassingspagina, zie de optie "Users List Location". Wanneer de locatie is ingesteld op iets anders dan Off, verschijnt er een selectievakje "Include past commenters" eronder.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; alt='Gebruikerslijstlocatie ingesteld op Rechts, met het selectievakje \'Inclusief eerdere commentatoren\' eronder weergegeven'; title='Instellingen gebruikerslijst'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

Bij meer dan 500 live gebruikers is de lijst tot 30 seconden verouderd.

---