[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

Standaard toont FastComments geen lijst met gebruikers op de pagina.

Je kunt een lijst weergeven van mensen die momenteel de pagina bekijken, naast de commentaarwidget. De lijst wordt live bijgewerkt wanneer gebruikers komen en gaan, en toont hun naam, avatar en een online-indicator.

Er zijn drie lay-outopties:

- `1` - Boven: een horizontale rij overlappende avatars die boven de reacties wordt weergegeven.
- `2` - Links: een zijbalk met namen en online-indicatoren die links van de widget worden weergegeven.
- `3` - Rechts: dezelfde zijbalk die rechts van de widget wordt weergegeven.

Stel de **usersListLocation** vlag in om de functie in te schakelen:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

Standaard toont de lijst alleen gebruikers die momenteel online zijn. Om ook mensen op te nemen die in het verleden op de pagina hebben gereageerd (maar de pagina nu niet bekijken), zet **usersListIncludeOffline** op true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Eerdere reageerders worden weergegeven zonder het groene online-bolletje, zodat duidelijk is wie er nu aanwezig is.

Gebruikers met privéprofielen worden weergegeven met een generieke avatar en een "Privéprofiel"-label, zodat het aantal nauwkeurig blijft zonder identiteiten te onthullen.

Dit kan ook zonder code worden geconfigureerd. Op de pagina voor het aanpassen van de widget, zie de optie "Locatie gebruikerslijst". Wanneer de locatie op iets anders dan "Uit" is ingesteld, verschijnt er onder die optie een selectievakje "Inclusief eerdere reageerders".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

---